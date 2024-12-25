#[derive(Debug, Clone, PartialEq, Default)]
pub struct DocumentStatus {
    pub current_line: usize,
    pub total_line: usize,
    pub filename: String,
    pub is_modified: bool,
}

impl DocumentStatus {
    pub fn modified_indicator_to_string(&self) -> String {
        if self.is_modified {
            String::from("(modified)")
        } else {
            String::new()
        }
    }
    pub fn line_count_to_string(&self) -> String {
        format!("{} lines", self.total_line)
    }
    pub fn position_indicator_to_string(&self) -> String {
        format!(
            "{}/{}",
            self.current_line.saturating_add(1),
            self.total_line
        )
    }
}
