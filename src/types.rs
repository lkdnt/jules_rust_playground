/// Core engine constants and common types

/// The logical width of the window
pub const WINDOW_WIDTH: u32 = 800;
/// The logical height of the window
pub const WINDOW_HEIGHT: u32 = 600;

/// The fixed width of the internal pixel buffer
pub const WIDTH: u32 = 400;
/// The fixed height of the internal pixel buffer
pub const HEIGHT: u32 = 300;

/// Number of fixed updates per second
pub const TICK_RATE: f64 = 60.0;
/// The fixed timestep for each update
pub const TIME_STEP: f64 = 1.0 / TICK_RATE;

/// A Simple 2D Vector type to avoid bringing in a math library
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
