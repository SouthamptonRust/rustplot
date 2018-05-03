/// Trait implemented by all drawable charts providing the interface for drawing functionality.
pub trait Chart {
    /// Draws the chart specified for the instance that this function is called on.
    fn draw(&self);
}
