use crate::terminal::Position;

#[derive(Default, Debug, Clone, Copy)]
pub struct Location {
  pub x: usize,
  pub y: usize,
}

impl From<Location> for Position {
    fn from(pos: Location) -> Self {
        Self {
          col: pos.x,
          row: pos.y,
        }
    }
}

impl Location {
    pub const fn subtract(&self, other: &Self) -> Self {
      Self {
        x: self.x.saturating_sub(other.x),
        y: self.y.saturating_sub(other.y),
      }
    }
}