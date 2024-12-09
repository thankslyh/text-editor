#[derive(Clone, Debug)]
pub struct Buffer {
  pub lines: Vec<String>,
}

impl Default for Buffer {
    fn default() -> Self {
        Buffer {
          lines: vec!["Hello World!".to_string()]
        }
    }
}