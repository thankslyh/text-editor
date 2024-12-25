use crate::size::Size;

pub trait UIComponent {
    fn set_size(&mut self, size: Size);

    fn mark_redraw(&mut self, redraw: bool);

    fn needs_redraw(&self) -> bool;

    fn resize(&mut self, size: Size) {
        self.mark_redraw(true);
        self.set_size(size);
    }

    fn render(&mut self, origin_y: usize) {
        if self.needs_redraw() {
            match self.draw(origin_y) {
                Ok(()) => self.mark_redraw(false),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not render component: {err:?}");
                    }
                }
            }
        }
    }

    fn draw(&mut self, origin_y: usize) -> Result<(), std::io::Error>;
}
