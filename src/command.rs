use core::convert::TryFrom;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use crate::size::Size;

#[derive(Debug, Clone, Copy)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
    PageUp,
    PageDown,
    Home,
    End,
}

impl TryFrom<KeyEvent> for Move {
    type Error = String;
    fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = value;
        if modifiers == KeyModifiers::NONE {
            match code {
                KeyCode::Up => Ok(Self::Up),
                KeyCode::Down => Ok(Self::Down),
                KeyCode::End => Ok(Self::End),
                KeyCode::Left => Ok(Self::Left),
                KeyCode::Home => Ok(Self::Home),
                KeyCode::Right => Ok(Self::Right),
                KeyCode::PageUp => Ok(Self::PageUp),
                KeyCode::PageDown => Ok(Self::PageDown),
                _ => Err(format!(
                    "Unsupported key code {code:?} or modifier {modifiers:?}"
                )),
            }
        } else {
            Err(format!(
                "Unsupported key code {code:?} or modifier {modifiers:?}"
            ))
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum System {
    Save,
    Resize(Size),
    Quit,
    Dismiss,
}

impl TryFrom<KeyEvent> for System {
    type Error = String;
    fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
        let KeyEvent {
            code, modifiers, ..
        } = value;
        match (code, modifiers) {
            (KeyCode::Char('s'), KeyModifiers::CONTROL) => Ok(Self::Save),
            (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(Self::Quit),
            (KeyCode::Esc, KeyModifiers::NONE) => Ok(Self::Dismiss),
            _ => Err(format!(
                "Unsupported key code {code:?} or modifier {modifiers:?}"
            )),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Edit {
    Delete,
    Insert(char),
    InsertNewline,
    DeleteBackward,
}

impl TryFrom<KeyEvent> for Edit {
    type Error = String;
    fn try_from(event: KeyEvent) -> Result<Self, Self::Error> {
        match (event.code, event.modifiers) {
            (KeyCode::Char(character), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                Ok(Self::Insert(character))
            }
            (KeyCode::Tab, KeyModifiers::NONE) => Ok(Self::Insert('\t')),
            (KeyCode::Enter, KeyModifiers::NONE) => Ok(Self::InsertNewline),
            (KeyCode::Backspace, KeyModifiers::NONE) => Ok(Self::DeleteBackward),
            (KeyCode::Delete, KeyModifiers::NONE) => Ok(Self::Delete),
            _ => Err(format!(
                "Unsupported key code {:?} with modifiers {:?}",
                event.code, event.modifiers
            )),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Command {
    Move(Move),
    Edit(Edit),
    System(System),
}

impl TryFrom<Event> for Command {
    type Error = String;
    fn try_from(value: Event) -> Result<Self, Self::Error> {
        match value {
            Event::Resize(w_u16, h_u16) => {
                let width = w_u16 as usize;
                let height = h_u16 as usize;
                let sys = System::Resize(Size { width, height });
                Ok(Self::System(sys))
            }
            Event::Key(key_ev) => Edit::try_from(key_ev)
                .map(Command::Edit)
                .or_else(|_| Move::try_from(key_ev).map(Command::Move))
                .or_else(|_| System::try_from(key_ev).map(Command::System))
                .map_err(|e| format!("Event not supported: {e:?}")),
            _ => Err(format!("Event not supported: {value:?}")),
        }
    }
}
