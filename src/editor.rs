use std::{cmp::min, io::Error};
use crossterm::event::{ read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers };
use crate::{terminal::{Position, Size}, view::View};

use super::terminal::Terminal;

#[derive(Default, Clone, Copy)]
pub struct Location {
  pub x: usize,
  pub y: usize,
}

pub struct Editor {
  pub quit: bool,
  pub location: Location,
  pub view: View,
}

impl Editor {

  pub fn run(&mut self)  {
    Terminal::initialize().unwrap();
    let res = self.repl();
    Terminal::terminate().unwrap();
    res.unwrap();
  }

  pub fn repl(&mut self) -> Result<(), Error> {
    loop {
      self.refresh_screen()?;
      if self.quit {
        break;
      }
      if let Event::Key(KeyEvent { code, modifiers, kind: KeyEventKind::Press, state }) = read()? {
        match code {
            KeyCode::Char('q') if modifiers == KeyModifiers::CONTROL => {
              self.quit = true
            },
            KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageDown
                | KeyCode::PageUp
                | KeyCode::End
                | KeyCode::Home => {
                  self.move_point(&code)?;
                },
            _ => ()
        }
      }
    }
    Ok(())
  }

  pub fn move_point(&mut self, key_code: &KeyCode) -> Result<(), Error> {
    let Location { mut x, mut y } = self.location;
    let Size { width, height } = Terminal::size()?;
    match key_code {
        KeyCode::Up => {
          y = y.saturating_sub(1)
        }
        KeyCode::Down => {
          y = min(y.saturating_add(1), height.saturating_sub(1));
        }
        KeyCode::Left => {
          x = x.saturating_sub(1)
        }
        KeyCode::Right => {
          x = min(x.saturating_add(1), width.saturating_sub(1))
        }
        KeyCode::PageDown => {
          y = height.saturating_sub(1)
        }
        KeyCode::PageUp => {
          y = 0
        }
        KeyCode::Home => {
          x = 0;
        }
        KeyCode::End => {
          x = width.saturating_sub(1);
        }
        _ => ()
    }
    self.location = Location {
      x,
      y
    };
    Ok(())
  }

  fn refresh_screen(&self) -> Result<(), Error> {
    Terminal::hide_caret()?;
    Terminal::move_caret_to(&Position::default())?;
    if self.quit {
      Terminal::clear_screen()?;
      print!("Goodbye.\r\n");
    } else {
      self.view.render()?;
      Terminal::move_caret_to(&Position {
        col: self.location.x,
        row: self.location.y,
      })?;
    }
    Terminal::show_caret()?;
    Terminal::execute()?;
    Ok(())
  }
}