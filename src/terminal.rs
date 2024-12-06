use crossterm::{cursor::{Hide, MoveTo, Show}, queue, style::Print, terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType}, Command};
use std::io::{stdout, Error, Write};

#[derive(Copy, Clone)]
pub struct Size {
  pub width: usize,
  pub height: usize,
}

#[derive(Default, Copy, Clone)]
pub struct Position {
  pub col: usize,
  pub row: usize,
}

pub struct Terminal;

impl Terminal {
  pub fn clear_line() -> Result<(), Error> {
    Self::queue_comand(Clear(ClearType::CurrentLine))
  }

  pub fn execute() -> Result<(), Error> {
    stdout().flush()?;
    Ok(())
  }

  pub fn hide_caret() -> Result<(), Error> {
    Self::queue_comand(Hide)
  }

  pub fn show_caret() -> Result<(), Error> {
    Self::queue_comand(Show)
  }

  pub fn print(str: &str) -> Result<(), Error> {
    Self::queue_comand(Print(str))
  }

  pub fn initialize() -> Result<(), Error> {
    enable_raw_mode()?;
    Self::clear_screen()?;
    Ok(())
}

  pub fn clear_screen() -> Result<(), Error> {
    Self::queue_comand(Clear(ClearType::All))
  }

  pub fn terminate() -> Result<(), Error> {
    disable_raw_mode()
  }
  pub fn move_caret_to(pos: &Position) -> Result<(), Error> {
    #[allow(clippy::as_conversions)]
    queue!(stdout(), MoveTo(pos.col as u16, pos.row as u16))?;
    Ok(())
  }

  pub fn size() -> Result<Size, Error> {
    let (w_u16, h_u16) = size()?;
    #[allow(clippy::as_conversions)]
    let width = w_u16 as usize;
    #[allow(clippy::as_conversions)]
    let height = h_u16 as usize;
    Ok(Size {
      width,
      height,
    })
  }

  pub fn queue_comand<T: Command>(command: T) -> Result<(), Error> {
    queue!(stdout(), command)?;
    Ok(())
  }
}