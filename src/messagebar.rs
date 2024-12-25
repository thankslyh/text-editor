use std::time::{Duration, Instant};

use crate::{size::Size, terminal::Terminal, uicomponent::UIComponent};

static DEFAULT_DURATION: Duration = Duration::new(5, 0);

pub struct Message {
    content: String,
    time: Instant,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            content: String::new(),
            time: Instant::now(),
        }
    }
}

impl Message {
    pub fn is_expired(&self) -> bool {
        Instant::now().duration_since(self.time) > DEFAULT_DURATION
    }
}

#[derive(Default)]
pub struct MessageBar {
    message: Message,
    need_redraw: bool,
    cleared_after_expiry: bool,
}

impl MessageBar {
    pub fn update_message(&mut self, msg: &str) {
        self.message = Message {
            content: msg.to_string(),
            time: Instant::now(),
        };
        self.cleared_after_expiry = false;
        self.mark_redraw(true);
    }
}

impl UIComponent for MessageBar {
    fn mark_redraw(&mut self, redraw: bool) {
        self.need_redraw = redraw;
    }

    fn needs_redraw(&self) -> bool {
        (!self.cleared_after_expiry && !self.message.is_expired()) || self.need_redraw
    }

    fn set_size(&mut self, _: Size) {}

    fn draw(&mut self, origin_y: usize) -> Result<(), std::io::Error> {
        if self.message.is_expired() {
            self.cleared_after_expiry = true;
        }
        let message = if self.message.is_expired() {
            ""
        } else {
            &self.message.content
        };
        Terminal::print_row(origin_y, &message)
    }
}
