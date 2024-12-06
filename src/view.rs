use std::io::Error;

use crate::terminal::{Size, Terminal};

static NAME: &str = env!("CARGO_PKG_NAME");
static VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {}

impl View {
    pub fn render() -> Result<(), Error> {
      let Size { height, .. } = Terminal::size()?;
      Terminal::clear_line()?;
      for current_row in 1..height {
          if current_row == height / 3 {
            Self::draw_welcome_message()?;
          } else {
            Self::draw_empty_line()?;
          }
      }
      Ok(())
    }

    pub fn draw_empty_line() -> Result<(), Error> {
      Terminal::print("~")?;
      Ok(())
    }

    pub fn draw_welcome_message() -> Result<(), Error> {
      let Size { width, .. } = Terminal::size()?;
      let mut message = format!("{NAME} editor -- version {VERSION}");
      let len = message.len();
      let padding = width.saturating_mul(len) / 2;
      let spaces = " ".repeat(padding.saturating_sub(1));
      message = format!("~{spaces}{message}");
      message.truncate(width);
      Terminal::print(&message)?;
      Ok(())
    }
}