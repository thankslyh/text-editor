use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use crate::terminal::Size;

pub enum Direction {
  PageUp,
  PageDown,
  Home,
  End,
  Up,
  Left,
  Right,
  Down,
}

pub enum EditorCommand {
  Move(Direction),
  Resize(Size),
  Quit,
}

impl TryFrom<Event> for EditorCommand {
    type Error = String;
    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent { code, modifiers, .. }) => match (code, modifiers) {
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(EditorCommand::Quit),
                (KeyCode::Up, _) => Ok(Self::Move(Direction::Up)),
                (KeyCode::Down, _) => Ok(Self::Move(Direction::Down)),
                (KeyCode::Left, _) => Ok(Self::Move(Direction::Left)),
                (KeyCode::Right, _) => Ok(Self::Move(Direction::Right)),
                (KeyCode::PageDown, _) => Ok(Self::Move(Direction::PageDown)),
                (KeyCode::PageUp, _) => Ok(Self::Move(Direction::PageUp)),
                (KeyCode::Home, _) => Ok(Self::Move(Direction::Home)),
                (KeyCode::End, _) => Ok(Self::Move(Direction::End)),
                _ => Err(format!("Key Code not supported: {code:?}")),
            },
            Event::Resize(w_u16, h_u16) => {
              #[allow(clippy::as_conversions)]
              let width = w_u16 as usize;
              #[allow(clippy::as_conversions)]
              let height = h_u16 as usize;
              Ok(Self::Resize(Size {width, height}))
            },
            _ => Err(format!("Event not supported: {event:?}")),
        }
    }
}