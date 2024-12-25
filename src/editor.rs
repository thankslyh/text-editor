use super::terminal::Terminal;
use crate::{
    command::{Command, Edit, System},
    commandbar::CommandBar,
    messagebar::MessageBar,
    position::Position,
    size::Size,
    statusbar::Statusbar,
    uicomponent::UIComponent,
    view::View,
};
use crossterm::event::{read, Event, KeyEvent, KeyEventKind};
use std::{
    io::Error,
    panic::{set_hook, take_hook},
};
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
const QUIT_TIMES: u8 = 3;
#[derive(Default)]
pub struct Editor {
    pub quit: bool,
    pub view: View,
    message_bar: MessageBar,
    command_bar: Option<CommandBar>,
    terminal_size: Size,
    pub status_bar: Statusbar,
    pub title: String,
    quit_times: u8,
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let default_hook = take_hook();
        set_hook(Box::new(move |panic_info| {
            let _ = Terminal::terminate();
            default_hook(panic_info);
        }));
        Terminal::initialize()?;
        let size = Terminal::size().unwrap_or_default();
        let mut editor = Self::default();
        editor.resize(size);
        let args: Vec<String> = std::env::args().collect();
        if let Some(file_name) = args.get(1) {
            println!("{}", file_name);
            if editor.view.load(file_name).is_err() {
                editor
                    .message_bar
                    .update_message(&format!("File open error, filename {}", file_name));
            }
        }
        editor
            .message_bar
            .update_message("HELP: Ctrl-S = save | Ctrl-Q = quit");
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

    pub fn process_command(&mut self, command: Command) {
        match command {
            Command::System(System::Quit) => {
                if self.command_bar.is_none() {
                    self.handler_quit();
                }
            }
            Command::System(System::Resize(size)) => self.resize(size),
            _ => self.reset_quit_times(),
        }

        match command {
            Command::System(System::Quit | System::Resize(_)) => {}
            Command::System(System::Dismiss) => {
                if self.command_bar.is_some() {
                    self.dimiss_prompt();
                    self.message_bar.update_message("File save abort!");
                }
            }
            Command::System(System::Save) => {
                if self.command_bar.is_none() {
                    self.handler_save()
                }
            }
            Command::Move(direction) => {
                if self.command_bar.is_none() {
                    self.view.move_text_location(direction)
                }
            }
            Command::Edit(edit) => {
                if let Some(command_bar) = &mut self.command_bar {
                    if matches!(edit, Edit::InsertNewline) {
                        let filename = command_bar.value();
                        self.dimiss_prompt();
                        self.save(Some(filename));
                    } else {
                        command_bar.handle_command_edit(edit);
                    }
                } else {
                    self.view.handler_edit(edit)
                }
            }
        }
    }

    pub fn evaluate_event(&mut self, ev: Event) {
        let should_handler = match ev {
            Event::Key(KeyEvent { kind, .. }) => kind == KeyEventKind::Press,
            Event::Resize(_, _) => true,
            _ => false,
        };
        if should_handler {
            if let Ok(command) = Command::try_from(ev) {
                self.process_command(command);
            }
        }
    }

    pub fn dimiss_prompt(&mut self) {
        self.command_bar = None;
        self.message_bar.mark_redraw(true);
    }

    pub fn handler_save(&mut self) {
        if self.view.is_file_loaded() {
            self.save(None);
        } else {
            self.show_prompt();
        }
    }

    pub fn save(&mut self, filename: Option<String>) {
        let result = if let Some(name) = filename {
            self.view.save_as(&name)
        } else {
            self.view.save()
        };
        if result.is_ok() {
            self.message_bar.update_message("File saved successfully.");
        } else {
            self.message_bar.update_message("Error writing file!");
        }
    }

    pub fn show_prompt(&mut self) {
        let mut command_bar = CommandBar::default();
        command_bar.set_prompt("Save as: ".to_string());
        command_bar.resize(Size {
            width: self.terminal_size.width,
            height: 1,
        });
        command_bar.mark_redraw(true);
        self.command_bar = Some(command_bar);
    }

    pub fn handler_quit(&mut self) {
        let is_modified = self.view.get_status().is_modified;
        if !is_modified || self.quit_times + 1 == QUIT_TIMES {
            self.quit = true;
        } else if is_modified {
            self.message_bar.update_message(&format!(
                "WARNING! File has unsaved changes. Press Ctrl-Q {} more times to quit.",
                QUIT_TIMES - self.quit_times - 1
            ));
            self.quit_times += 1;
        }
    }

    fn reset_quit_times(&mut self) {
        if self.quit_times > 0 {
            self.quit_times = 0;
            self.message_bar.update_message("");
        }
    }

    fn resize(&mut self, size: Size) {
        self.terminal_size = size;
        self.view.resize(Size {
            height: size.height.saturating_sub(2),
            width: size.width,
        });
        self.message_bar.resize(Size {
            height: 1,
            width: size.width,
        });
        self.status_bar.resize(Size {
            height: 1,
            width: size.width,
        });
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret();
        let Size { height, width } = self.terminal_size;
        if width == 0 || height == 0 {
            return;
        }
        let bottom_bar_row = height.saturating_sub(1);
        if let Some(command_bar) = &mut self.command_bar {
            command_bar.render(bottom_bar_row);
        } else {
            self.message_bar.render(bottom_bar_row);
        }
        if height > 1 {
            self.status_bar.render(height.saturating_sub(2));
        }
        if height > 2 {
            self.view.render(0);
        }
        let new_caret = if let Some(command_bar) = &self.command_bar {
            Position {
                row: bottom_bar_row,
                col: command_bar.caret_position_col(),
            }
        } else {
            self.view.text_location_to_postion()
        };
        let _ = Terminal::move_caret_to(&new_caret);
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
