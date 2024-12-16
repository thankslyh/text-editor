use crate::{
    buffer::Buffer,
    documentstatus::DocumentStatus,
    editor::{NAME, VERSION},
    editorcommand::{Direction, EditorCommand},
    line::Line,
    terminal::{Position, Size, Terminal},
};
use std::cmp::min;

#[derive(Copy, Clone, Default)]
pub struct Location {
    pub grapheme_index: usize,
    pub line_index: usize,
}

pub struct View {
    buf: Buffer,
    size: Size,
    need_redraw: bool,
    margin_bottom: usize,
    text_location: Location,
    scroll_offset: Position,
}

impl Default for View {
    fn default() -> Self {
        Self {
            buf: Buffer::default(),
            size: Terminal::size().unwrap_or_default(),
            need_redraw: true,
            margin_bottom: 0,
            text_location: Location::default(),
            scroll_offset: Position::default(),
        }
    }
}

impl View {
    pub fn new(margin_bottom: usize) -> Self {
        let Size { width, height } = Terminal::size().unwrap_or_default();
        Self {
            buf: Buffer::default(),
            size: Size {
                width,
                height: height.saturating_sub(margin_bottom),
            },
            margin_bottom,
            need_redraw: true,
            text_location: Location::default(),
            scroll_offset: Position::default(),
        }
    }

    pub fn get_status(&self) -> DocumentStatus {
        DocumentStatus {
            current_line: self.text_location.line_index,
            total_line: self.buf.height(),
            is_modify: false,
            filename: self.buf.file_info.to_string(),
            is_modified: self.need_redraw,
        }
    }
    pub fn load(&mut self, filename: &str) {
        if let Ok(buf) = Buffer::read_file(filename) {
            self.buf = buf;
            self.need_redraw = true;
        }
    }

    pub fn resize(&mut self, size: Size) {
        self.size = Size {
            width: size.width,
            height: size.height.saturating_sub(self.margin_bottom),
        };
        self.scroll_text_location_into_view();
        self.need_redraw = true;
    }

    pub fn render_line(at: usize, line: &str) {
        let result = Terminal::print_row(at, line);
        debug_assert!(result.is_ok(), "Failed to render line");
    }

    pub fn render(&mut self) {
        if !self.need_redraw || self.size.height == 0 {
            return;
        }
        let Size { width, height } = self.size;
        if width == 0 || height == 0 {
            return;
        }
        let top = self.scroll_offset.row;
        #[allow(clippy::as_conversions)]
        let ver_center = height / 3;
        for current_row in 1..height {
            if let Some(line) = self.buf.lines.get(current_row.saturating_add(top)) {
                let left = self.scroll_offset.col;
                let right = self.scroll_offset.col.saturating_add(width);
                Self::render_line(current_row, &line.get(left..right));
            } else if current_row == ver_center && self.buf.is_empty() {
                Self::render_line(current_row, &Self::buid_welcome_message(width));
            } else {
                Self::render_line(current_row, "~");
            }
        }
        self.need_redraw = false;
    }

    pub fn handler_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::Move(d) => self.move_text_location(d),
            EditorCommand::Resize(size) => {
                self.resize(size);
            }
            EditorCommand::Insert(c) => {
                self.insert_char(c);
            }
            EditorCommand::Backspace => {
                self.backspace();
            }
            EditorCommand::Delete => {
                self.delete_backward();
            }
            EditorCommand::Enter => {
                self.insert_new_line();
            }
            EditorCommand::Save => {
                self.save();
            }
            _ => {}
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
        self.need_redraw = true
    }

    pub fn insert_new_line(&mut self) {
        self.buf.insert_new_line(self.text_location);
        self.move_text_location(Direction::Right);
        self.need_redraw = true;
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
        self.need_redraw = true;
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
        self.need_redraw = self.need_redraw || changed
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
        self.need_redraw = self.need_redraw || changed
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

    pub fn move_text_location(&mut self, d: Direction) {
        let Size { height, .. } = self.size;
        let _ = match d {
            Direction::Up => self.move_up(1),
            Direction::Down => self.move_down(1),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
            Direction::PageDown => self.move_down(height.saturating_sub(1)),
            Direction::PageUp => self.move_up(height.saturating_sub(1)),
            Direction::Home => self.move_start_of_line(),
            Direction::End => self.move_end_of_line(),
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

    fn save(&mut self) {
        let _ = self.buf.save();
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
