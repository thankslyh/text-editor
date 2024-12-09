use std::io::Error;

use crate::{buffer::Buffer, terminal::{Size, Terminal}};

static NAME: &str = env!("CARGO_PKG_NAME");
static VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default, Debug)]
pub struct View {
  buf: Buffer
}

impl View {
    pub fn get_buf(&self) -> Buffer {
      return self.buf.clone()
    }
    pub fn render(&self) -> Result<(), Error> {
      let Size { height, .. } = Terminal::size()?;
      for current_row in 1..height {
        Terminal::clear_line()?;
        if let Some(line) = self.buf.lines.get(current_row) {
          Terminal::print(line)?;
          Terminal::print("\r\n")?;
          continue;
        }
        #[allow(clippy::integer_division)]
        if current_row == height / 3 {
          self.draw_welcome_message()?;
        } else {
          self.draw_empty_line()?;
        }
        if current_row.saturating_add(1) < height {
          Terminal::print("\r\n")?;
        }
      }
      Ok(())
    }

    pub fn draw_empty_line(&self) -> Result<(), Error> {
      Terminal::print("~")?;
      Ok(())
    }

    pub fn draw_welcome_message(&self) -> Result<(), Error> {
      let Size { width, .. } = Terminal::size()?;
      let mut message = format!("{NAME} editor -- version {VERSION}");
      let len = message.len();
      let padding = (width.saturating_sub(len)) / 2;
      let spaces = " ".repeat(padding.saturating_sub(1));
      message = format!("~{spaces}{message}");
      message.truncate(width);
      Terminal::print(&message)?;
      Ok(())
    }
}