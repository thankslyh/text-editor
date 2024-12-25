use crate::{
    buffer::Buffer,
    command::{Edit, Move},
    documentstatus::DocumentStatus,
    editor::{NAME, VERSION},
    line::Line,
    position::Position,
    size::Size,
    terminal::Terminal,
    uicomponent::UIComponent,
};
use std::{cmp::min, io::Error};

#[derive(Copy, Clone, Default)]
pub struct Location {
    pub grapheme_index: usize,
    pub line_index: usize,
}

#[derive(Default)]
pub struct View {
    buf: Buffer,
    size: Size,
    need_redraw: bool,
    text_location: Location,
    scroll_offset: Position,
}

impl View {
    pub fn get_status(&self) -> DocumentStatus {
        DocumentStatus {
            current_line: self.text_location.line_index,
            total_line: self.buf.height(),
            filename: self.buf.file_info.to_string(),
            is_modified: self.buf.is_modify,
        }
    }
    pub fn load(&mut self, filename: &str) -> Result<(), Error> {
        let buf = Buffer::read_file(filename)?;
        self.buf = buf;
        self.mark_redraw(true);
        Ok(())
    }

    pub fn is_file_loaded(&self) -> bool {
        self.buf.is_file_loaded()
    }

    pub fn render_line(at: usize, line: &str) -> Result<(), Error> {
        Terminal::print_row(at, line)
    }

    pub fn handler_edit(&mut self, edit: Edit) {
        match edit {
            Edit::Delete => self.delete_backward(),
            Edit::Insert(c) => self.insert_char(c),
            Edit::InsertNewline => self.insert_new_line(),
            Edit::DeleteBackward => self.backspace(),
        }
    }

    pub fn insert_char(&mut self, s: char) {
        let Location { line_index, .. } = self.text_location;
        let old_width = self.buf.lines.get(line_index).map_or(0, Line::len);
        self.buf.insert_char(s, self.text_location);
        let new_width = self.buf.lines.get(line_index).map_or(0, Line::len);
        let grapheme_delta = new_width.saturating_sub(old_width);
        if grapheme_delta > 0 {
            self.move_right();
        }
        self.mark_redraw(true);
    }

    pub fn insert_new_line(&mut self) {
        self.buf.insert_new_line(self.text_location);
        self.move_text_location(Move::Right);
        self.mark_redraw(true);
    }

    pub fn backspace(&mut self) {
        self.move_left();
        self.delete_backward();
    }

    pub fn delete_backward(&mut self) {
        if self.text_location.line_index == 0 && self.text_location.grapheme_index == 0 {
            return;
        }
        self.buf.delete(self.text_location);
        self.mark_redraw(true);
    }

    fn scroll_vertically(&mut self, to: usize) {
        let height = self.size.height;
        let changed = if to < self.scroll_offset.row {
            self.scroll_offset.row = to;
            true
        } else if to >= self.scroll_offset.row.saturating_add(height) {
            self.scroll_offset.row = to.saturating_sub(height).saturating_add(1);
            true
        } else {
            false
        };
        if changed {
            self.mark_redraw(true);
        }
    }

    fn scroll_heriztion(&mut self, to: usize) {
        let width = self.size.width;
        let changed = if to < self.scroll_offset.col {
            self.scroll_offset.col = to;
            true
        } else if to >= self.scroll_offset.col.saturating_add(width) {
            self.scroll_offset.col = to.saturating_sub(width).saturating_add(1);
            true
        } else {
            false
        };
        if changed {
            self.mark_redraw(true);
        }
    }

    pub fn text_location_to_postion(&self) -> Position {
        let Location {
            line_index,
            grapheme_index,
        } = self.text_location;
        let col = self
            .buf
            .lines
            .get(line_index)
            .map_or(0, |line| line.width_until(grapheme_index));
        Position {
            row: line_index,
            col,
        }
    }

    pub fn move_text_location(&mut self, mv: Move) {
        let Size { height, .. } = self.size;
        let _ = match mv {
            Move::Up => self.move_up(1),
            Move::Down => self.move_down(1),
            Move::Left => self.move_left(),
            Move::Right => self.move_right(),
            Move::PageDown => self.move_down(height.saturating_sub(1)),
            Move::PageUp => self.move_up(height.saturating_sub(1)),
            Move::Home => self.move_start_of_line(),
            Move::End => self.move_end_of_line(),
        };
        self.scroll_text_location_into_view();
    }

    pub fn scroll_text_location_into_view(&mut self) {
        let pos = self.text_location_to_postion();
        self.scroll_heriztion(pos.col);
        self.scroll_vertically(pos.row);
    }

    fn move_up(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_sub(step);
        self.snap_to_valid_grapheme();
    }

    fn move_down(&mut self, step: usize) {
        self.text_location.line_index = self.text_location.line_index.saturating_add(step);
        self.snap_to_valid_grapheme();
        self.snap_to_valid_line();
    }

    fn move_left(&mut self) {
        if self.text_location.grapheme_index > 0 {
            self.text_location.grapheme_index -= 1;
        } else if self.text_location.line_index > 0 {
            self.move_up(1);
            self.move_end_of_line();
        }
    }

    fn move_right(&mut self) {
        let width = self
            .buf
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::len);
        if self.text_location.grapheme_index < width {
            self.text_location.grapheme_index += 1;
        } else {
            self.move_start_of_line();
            self.move_down(1)
        }
    }

    fn move_start_of_line(&mut self) {
        self.text_location.grapheme_index = 0;
    }

    fn move_end_of_line(&mut self) {
        self.text_location.grapheme_index = self
            .buf
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::len);
    }

    fn snap_to_valid_grapheme(&mut self) {
        self.text_location.grapheme_index = self
            .buf
            .lines
            .get(self.text_location.line_index)
            .map_or(0, |line| min(self.text_location.grapheme_index, line.len()));
    }

    fn snap_to_valid_line(&mut self) {
        self.text_location.line_index = min(self.text_location.line_index, self.buf.height());
    }

    pub fn save(&mut self) -> Result<(), Error> {
        self.buf.save()
    }

    pub fn save_as(&mut self, filename: &str) -> Result<(), Error> {
        self.buf.save_as(filename)
    }

    pub fn buid_welcome_message(width: usize) -> String {
        if width == 0 {
            return String::new();
        }
        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        let remaining_width = width.saturating_sub(1);
        if width < len {
            return "~".to_string();
        }
        format!("{:<1}{:^remaining_width$}", "~", welcome_message)
    }
}

impl UIComponent for View {
    fn mark_redraw(&mut self, redraw: bool) {
        self.need_redraw = redraw;
    }

    fn needs_redraw(&self) -> bool {
        self.need_redraw
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
        self.scroll_text_location_into_view();
    }

    fn draw(&mut self, origin_y: usize) -> Result<(), std::io::Error> {
        let Size { width, height } = self.size;
        let end_y = origin_y.saturating_add(height);

        #[allow(clippy::as_conversions)]
        let top_third = height / 3;
        let scroll_top = self.scroll_offset.row;
        for current_row in origin_y..end_y {
            let line_idx = current_row
                .saturating_sub(origin_y)
                .saturating_add(scroll_top);
            if let Some(line) = self.buf.lines.get(line_idx) {
                let left = self.scroll_offset.col;
                let right = self.scroll_offset.col.saturating_add(width);
                Self::render_line(current_row, &line.get(left..right))?;
            } else if current_row == top_third && self.buf.is_empty() {
                Self::render_line(current_row, &Self::buid_welcome_message(width))?;
            } else {
                Self::render_line(current_row, "~")?;
            }
        }
        Ok(())
    }
}
