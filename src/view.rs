use crate::{buffer::Buffer, editorcommand::{Direction, EditorCommand}, line::Line, terminal::{Position, Size, Terminal}};
use std::cmp::min;
static NAME: &str = env!("CARGO_PKG_NAME");
static VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Copy, Clone, Default)]
pub struct Location {
  pub grapheme_index: usize,
  pub line_index: usize,
}

pub struct View {
  buf: Buffer,
  size: Size,
  need_redraw: bool,
  text_location: Location,
  scroll_offset: Position,
}


impl Default for View {
    fn default() -> Self {
        Self {
          buf: Buffer::default(),
          size: Terminal::size().unwrap_or_default(),
          need_redraw: true,
          text_location: Location::default(),
          scroll_offset: Position::default(),
        }
    }
}

impl View {

    pub fn load(&mut self, filename: &str) {
      if let Ok(buf) = Buffer::read_file(filename) {
        self.buf = buf;
        self.need_redraw = true;
      }
    }

    pub fn resize(&mut self, size: Size) {
      self.size = size;
      self.scroll_text_location_into_view();
      self.need_redraw = true;
    }

    pub fn render_line(at: usize, line: &str) {
      let result = Terminal::print_row(at, line);
      debug_assert!(result.is_ok(), "Failed to render line");
    }

    pub fn render(&mut self){
      if !self.need_redraw {
        return
      }
      let Size { width, height } = self.size;
      if width == 0 || height == 0 {
        return
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
          EditorCommand::Move(d) => {
            self.move_text_location(d)
          },
          EditorCommand::Resize(size) => {
            self.resize(size);
          },
          _ => {},
      }
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
      let Location { line_index, grapheme_index } = self.text_location;
      let col = self.buf.lines.get(line_index).map_or(0, |line| {
        line.width_until(grapheme_index)
      });
      Position {
        row: line_index,
        col
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
      } else {
        self.move_up(1);
        self.move_end_of_line();
      }
    }

    fn move_right(&mut self) {
      let width = self.buf.lines.get(self.text_location.line_index).map_or(0, Line::len);
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
      self.text_location.grapheme_index = self.buf.lines.get(self.text_location.line_index).map_or(0, Line::len);
    }

    fn snap_to_valid_grapheme(&mut self) {
      self.text_location.grapheme_index = self.buf.lines.get(self.text_location.line_index).map_or(0, |line| {
        min(self.text_location.grapheme_index, line.len())
      });
    }

    fn snap_to_valid_line(&mut self) {
      self.text_location.line_index = min(self.text_location.line_index, self.buf.height());
    }

    pub fn buid_welcome_message(width: usize) -> String {
      if width == 0 {
        return " ".to_string();
      }
      let welcome_message = format!("{NAME} editor -- version {VERSION}");
      let len = welcome_message.len();
      if width <= len {
        return "~".to_string();
      }
      // we allow this since we don't care if our welcome message is put _exactly_ in the middle.
      // it's allowed to be a bit to the left or right.
      #[allow(clippy::integer_division)]
      let padding = (width.saturating_sub(len).saturating_sub(1)) / 2;

      let mut full_message = format!("~{}{}", " ".repeat(padding), welcome_message);
      full_message.truncate(width);
      full_message
    }
}