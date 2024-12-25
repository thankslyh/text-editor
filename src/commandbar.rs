use crate::{command::Edit, line::Line, size::Size, terminal::Terminal, uicomponent::UIComponent};
use std::cmp::min;

#[derive(Default)]
pub struct CommandBar {
    size: Size,
    prompt: String,
    need_redraw: bool,
    value: Line,
}

impl CommandBar {
    pub fn handle_command_edit(&mut self, edit: Edit) {
        match edit {
            Edit::Insert(c) => self.value.append_char(c),
            Edit::Delete | Edit::InsertNewline => {}
            Edit::DeleteBackward => self.value.delete_last(),
        }
    }

    pub fn value(&self) -> String {
        self.value.to_string()
    }

    pub fn set_prompt(&mut self, prompt: String) {
        self.prompt = prompt
    }

    pub fn caret_position_col(&self) -> usize {
        let max_width = self.prompt.len().saturating_add(self.value.len());
        min(max_width, self.size.width)
    }
}

impl UIComponent for CommandBar {
    fn mark_redraw(&mut self, redraw: bool) {
        self.need_redraw = redraw;
    }

    fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    fn needs_redraw(&self) -> bool {
        self.need_redraw
    }

    fn draw(&mut self, origin_y: usize) -> Result<(), std::io::Error> {
        let val_width = self.size.width.saturating_sub(self.prompt.len());
        let val_end = self.value.width();
        let val_start = val_end.saturating_sub(val_width);
        let message = format!("{}{}", self.prompt, self.value.get(val_start..val_start));
        let to_print = if message.len() <= self.size.width {
            message
        } else {
            String::new()
        };
        Terminal::print_row(origin_y, &to_print)
    }
}
