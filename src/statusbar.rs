use crate::{
    documentstatus::DocumentStatus, size::Size, terminal::Terminal, uicomponent::UIComponent,
};

#[derive(Default)]
pub struct Statusbar {
    current_status: DocumentStatus,
    need_redraw: bool,
    margin_bottom: usize,
    size: Size,
    is_visible: bool,
}

impl Statusbar {
    pub fn update_status(&mut self, status: DocumentStatus) {
        if self.current_status != status {
            self.current_status = status;
            self.mark_redraw(true);
        }
    }
}

impl UIComponent for Statusbar {
    fn mark_redraw(&mut self, redraw: bool) {
        self.need_redraw = redraw;
    }

    fn needs_redraw(&self) -> bool {
        self.need_redraw
    }

    fn set_size(&mut self, size: Size) {
        self.size.width = size.width;
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
        self.size.height = position_y;
        self.is_visible = is_visible;
    }

    fn draw(&mut self, origin_y: usize) -> Result<(), std::io::Error> {
        let line_count = self.current_status.line_count_to_string();
        let modified_indicator = self.current_status.modified_indicator_to_string();
        let filename = self.current_status.filename.clone();
        let beginning = format!("{} {} {}", filename, modified_indicator, line_count);
        let position_indicator = self.current_status.position_indicator_to_string();
        let remainder_len = self.size.width.saturating_sub(beginning.len());
        let status = format!("{beginning}{position_indicator:>remainder_len$}");
        let to_print = if status.len() <= self.size.width {
            status
        } else {
            String::new()
        };
        Terminal::print_inverted_row(origin_y, &to_print)?;
        Ok(())
    }
}
