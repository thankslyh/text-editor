use crate::{
    documentstatus::DocumentStatus,
    terminal::{Size, Terminal},
};

pub struct Statusbar {
    current_status: DocumentStatus,
    need_redraw: bool,
    margin_bottom: usize,
    width: usize,
    position_y: usize,
    is_visible: bool,
}

impl Statusbar {
    pub fn new(margin_bottom: usize) -> Self {
        let size = Terminal::size().unwrap_or_default();
        let mut status_bar = Self {
            current_status: DocumentStatus::default(),
            margin_bottom,
            width: size.width,
            position_y: 0,
            need_redraw: true,
            is_visible: false,
        };
        status_bar.resize(size);
        status_bar
    }

    pub fn resize(&mut self, size: Size) {
        self.width = self.width;
        let mut position_y = 0;
        let mut is_visible = false;
        if let Some(result) = size
            .height
            .checked_sub(self.margin_bottom)
            .and_then(|result| result.checked_sub(1))
        {
            position_y = result;
            is_visible = true;
        }
        self.position_y = position_y;
        self.is_visible = is_visible;
        self.need_redraw = true
    }

    pub fn render(&mut self) {
        if !self.need_redraw || !self.is_visible {
            return;
        }
        if let Ok(size) = Terminal::size() {
            let line_count = self.current_status.line_count_to_string();
            let modified_indicator = self.current_status.modified_indicator_to_string();
            let beginning = format!(
                "{} - {line_count} {modified_indicator}",
                self.current_status.filename
            );
            let position_indicator = self.current_status.position_indicator_to_string();
            let remainder_len = size.width.saturating_sub(beginning.len());
            let status = format!("{beginning}{position_indicator:>remainder_len$}");
            let to_print = if status.len() <= size.width {
                status
            } else {
                String::new()
            };
            let result = Terminal::print_inverted_row(self.position_y, &to_print);
            debug_assert!(result.is_ok(), "Failed to render status bar");
            self.need_redraw = false;
        }
    }

    pub fn update_status(&mut self, status: DocumentStatus) {
        if self.current_status != status {
            self.current_status = status;
            self.need_redraw = true;
        }
    }
}
