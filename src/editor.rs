use std::{cmp::min, io::Error, panic::{set_hook, take_hook}};
use crossterm::event::{ self, read, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers };
use crate::{editorcommand::EditorCommand, terminal::{Position, Size}, view::View};
use super::terminal::Terminal;

pub struct Editor {
  pub quit: bool,
  pub view: View,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
          quit: false,
          view: View::default(),
        }
    }
}
impl Editor {

  pub fn new() -> Result<Self, Error> {
    let default_hook = take_hook();
    set_hook(Box::new(move |panic_info| {
      let _ = Terminal::terminate();
      default_hook(panic_info);
    }));
    Terminal::initialize()?;
    let mut view = View::default();
    let args: Vec<String> = std::env::args().collect();
    if let Some(file_name) = args.get(1) {
      view.load(&file_name);
    }
    Ok(Self {
      quit: false,
      view
    })
  }

  pub fn run(&mut self)  {
    loop {
        self.refresh_screen();
        if self.quit {
          break;
        }
        match read() {
            Ok(event) => self.evaluate_event(event),
            Err(err) => {
              #[cfg(debug_assertions)]
              {
                  panic!("Could not read event: {err:?}");
              }
            }
        }
    }

  }


  pub fn evaluate_event(&mut self, ev: Event) {
    let should_handler = match ev {
        Event::Key(KeyEvent {kind, ..}) => kind == KeyEventKind::Press,
        Event::Resize(_, _) => true,
        _ => false,
    };
    if should_handler {
      match EditorCommand::try_from(ev) {
          Ok(command) => {
            if matches!(command, EditorCommand::Quit) {
              self.quit = true
            } else {
              self.view.handler_command(command);
            }
          },
          Err(err) => {
            #[cfg(debug_assertions)]
            {
                panic!("Could not handle command: {err}");
            }
          },
      }
    }
  }

  fn refresh_screen(&mut self) {
    let _ = Terminal::hide_caret();
    self.view.render();
    let _ = Terminal::move_caret_to(&self.view.get_position());
    let _ = Terminal::show_caret();
    let _ = Terminal::execute();
  }
}

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.quit {
          let _ = Terminal::print("Goodbye!!!! \r\n");
        }
    }
}