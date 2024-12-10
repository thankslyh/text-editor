use crate::{buffer::Buffer, location::Location, editorcommand::{Direction, EditorCommand}, terminal::{Position, Size, Terminal}};

static NAME: &str = env!("CARGO_PKG_NAME");
static VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub struct View {
  buf: Buffer,
  size: Size,
  need_redraw: bool,
  location: Location,
  scroll_offset: Location,
}

impl Default for View {
    fn default() -> Self {
        Self {
          buf: Buffer::default(),
          size: Terminal::size().unwrap_or_default(),
          need_redraw: true,
          location: Location::default(),
          scroll_offset: Location::default(),
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
      self.scroll_location_into_view();
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
      let top = self.scroll_offset.y;
      #[allow(clippy::as_conversions)]
      let ver_center = height / 3;
      for current_row in 1..height {
        if let Some(line) = self.buf.lines.get(current_row.saturating_add(top)) {
          let left = self.scroll_offset.x;
          let right = self.scroll_offset.x.saturating_add(width);
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
    pub fn move_text_location(&mut self, d: Direction) {
      let Location { mut x, mut y } = self.location;
      let Size { width, height } = self.size;
      match d {
          Direction::Up => {
            y = y.saturating_sub(1)
          }
          Direction::Down => {
            y = y.saturating_add(1);
          }
          Direction::Left => {
            x = x.saturating_sub(1)
          }
          Direction::Right => {
            x = x.saturating_add(1);
          }
          Direction::PageDown => {
            y = height.saturating_sub(1)
          }
          Direction::PageUp => {
            y = 0
          }
          Direction::Home => {
            x = 0;
          }
          Direction::End => {
            x = width.saturating_sub(1);
          }
      }
      self.location = Location {
        x,
        y
      };
      self.scroll_location_into_view();
    }

    pub fn get_position(&mut self) -> Position {
      self.location.subtract(&self.scroll_offset).into()
    }

    pub fn scroll_location_into_view(&mut self) {
      let Location { x, y } = self.location;
      let Size { width, height } = self.size;
      let mut offset_changed = false;

      if x < self.scroll_offset.x {
        self.scroll_offset.x = x;
        offset_changed = true;
      } else if x >= self.scroll_offset.x.saturating_add(width) {
        self.scroll_offset.x = x.saturating_sub(width).saturating_add(1);
        offset_changed = true;
      }

      // Scroll vertically
      if y < self.scroll_offset.y {
        self.scroll_offset.y = y;
        offset_changed = true;
      } else if y >= self.scroll_offset.y.saturating_add(height) {
        self.scroll_offset.y = y.saturating_sub(height).saturating_add(1);
        offset_changed = true;
      }
      self.need_redraw = offset_changed
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