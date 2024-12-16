use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::{Attribute, Print},
    terminal::{
        disable_raw_mode, enable_raw_mode, size, Clear, ClearType, DisableLineWrap, EnableLineWrap,
        EnterAlternateScreen, LeaveAlternateScreen, SetTitle,
    },
    Command,
};
use std::io::{stdout, Error, Write};

#[derive(Default, Debug, Copy, Clone)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

#[derive(Default, Copy, Clone)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

impl Position {
    pub const fn saturating_sub(self, other: Self) -> Self {
        Self {
            col: self.col.saturating_sub(other.col),
            row: self.row.saturating_sub(other.row),
        }
    }
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

    pub fn disable_line_warp() -> Result<(), Error> {
        Self::queue_comand(DisableLineWrap)
    }

    pub fn enable_line_warp() -> Result<(), Error> {
        Self::queue_comand(EnableLineWrap)
    }

    pub fn set_title(title: &str) -> Result<(), Error> {
        Self::queue_comand(SetTitle(title))
    }

    pub fn print_inverted_row(row: usize, line_txt: &str) -> Result<(), Error> {
        let width = Self::size().unwrap_or_default().width;
        Self::print_row(
            row,
            &format!(
                "{}{:width$.width$}{}",
                Attribute::Reverse,
                line_txt,
                Attribute::Reset
            ),
        )
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
        Self::enter_alternate_screen()?;
        Self::enable_line_warp()?;
        Self::clear_screen()?;
        Self::execute()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_comand(Clear(ClearType::All))
    }

    pub fn terminate() -> Result<(), Error> {
        Self::leave_alternate_screen()?;
        Self::disable_line_warp()?;
        Self::show_caret()?;
        disable_raw_mode()?;
        Ok(())
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
        Ok(Size { width, height })
    }

    pub fn queue_comand<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }

    pub fn print_row(at: usize, line_txt: &str) -> Result<(), Error> {
        Terminal::move_caret_to(&Position { col: 0, row: at })?;
        Terminal::clear_line()?;
        Terminal::print(line_txt)?;
        Ok(())
    }

    pub fn leave_alternate_screen() -> Result<(), Error> {
        Self::queue_comand(LeaveAlternateScreen)?;
        Ok(())
    }

    pub fn enter_alternate_screen() -> Result<(), Error> {
        Self::queue_comand(EnterAlternateScreen)?;
        Ok(())
    }
}
