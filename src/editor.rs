use super::terminal::Terminal;
use crate::{editorcommand::EditorCommand, statusbar::Statusbar, view::View};
use crossterm::event::{read, Event, KeyEvent, KeyEventKind};
use std::{
    io::Error,
    panic::{set_hook, take_hook},
};
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    pub quit: bool,
    pub view: View,
    pub status_bar: Statusbar,
    pub title: String,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            quit: false,
            view: View::default(),
            title: String::new(),
            status_bar: Statusbar::new(1),
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
        let mut editor = Self {
            quit: false,
            view: View::new(2),
            title: NAME.to_string(),
            status_bar: Statusbar::new(1),
        };
        let args: Vec<String> = std::env::args().collect();
        if let Some(file_name) = args.get(1) {
            editor.view.load(&file_name);
        }
        editor.refresh_status();
        Ok(editor)
    }

    pub fn refresh_status(&mut self) {
        let status = self.view.get_status();
        let title = format!("{} - {NAME}", status.filename);
        self.status_bar.update_status(status);
        if title != self.title && matches!(Terminal::set_title(&title), Ok(())) {
            self.title = title
        }
    }

    pub fn run(&mut self) {
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
            let status = self.view.get_status();
            self.status_bar.update_status(status);
        }
    }

    pub fn evaluate_event(&mut self, ev: Event) {
        let should_handler = match ev {
            Event::Key(KeyEvent { kind, .. }) => kind == KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };
        if should_handler {
            if let Ok(command) = EditorCommand::try_from(ev) {
                if matches!(command, EditorCommand::Quit) {
                    self.quit = true
                } else {
                    self.view.handler_command(command);
                    if let EditorCommand::Resize(size) = command {
                        self.status_bar.resize(size);
                    }
                }
            }
        }
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        self.view.render();
        self.status_bar.render();
        let _ = Terminal::move_caret_to(&self.view.text_location_to_postion());
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
