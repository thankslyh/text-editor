use std::fs;

use crate::line::Line;

#[derive(Debug)]
pub struct Buffer {
  pub lines: Vec<Line>,
}

impl Default for Buffer {
    fn default() -> Self {
        Buffer {
          lines: Vec::new()
        }
    }
}

impl Buffer {
  pub fn read_file(filepath: &str) -> Result<Self, std::io::Error> {
    let contents = fs::read_to_string(filepath)?;
    let mut lines = Vec::new();
    for str in contents.lines() {
        lines.push(Line::from(str));
    }
    Ok(Self {
      lines
    })
  }

  pub fn is_empty(&self) -> bool {
    self.lines.is_empty()
  }
}