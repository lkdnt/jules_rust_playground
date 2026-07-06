/// Rendering module
/// Modifies the `pixels` frame buffer based on the game state (World).

use crate::ecs::{World, COMPONENT_RENDERABLE, COMPONENT_TRANSFORM};
use crate::types::{HEIGHT, WIDTH};

/// Clears the pixel buffer with a solid color.
pub fn clear(frame: &mut [u8], color: [u8; 4]) {
    // Branchless fast clear using chunks
    for pixel in frame.chunks_exact_mut(4) {
        pixel.copy_from_slice(&color);
    }
}

/// Renders all entities with Transform and Renderable components.
/// Driven by Data-Oriented Design: Iterate over arrays and use bitmasks to filter.
pub fn draw_world(frame: &mut [u8], world: &World) {
    let mask = COMPONENT_TRANSFORM | COMPONENT_RENDERABLE;

    for i in 0..world.signatures.len() {
        // Branchless check: if (sig & mask) == mask, the entity has both components.
        // If not, we can skip it. This is a single branch instead of multiple component existence checks.
        if (world.signatures[i] & mask) == mask {
            let transform = &world.transforms[i];
            let renderable = &world.renderables[i];

            draw_rect(
                frame,
                transform.position.x as i32,
                transform.position.y as i32,
                renderable.size.x as i32,
                renderable.size.y as i32,
                renderable.color,
            );
        }
    }
}

/// Draws a colored rectangle into the pixel buffer.
/// Fast and simple block drawing.
fn draw_rect(frame: &mut [u8], x: i32, y: i32, w: i32, h: i32, color: [u8; 4]) {
    // Early exit for out of bounds
    if w <= 0 || h <= 0 || x >= WIDTH as i32 || y >= HEIGHT as i32 || x + w <= 0 || y + h <= 0 {
        return;
    }

    // Clamp coordinates to buffer bounds
    let start_x = x.max(0) as usize;
    let start_y = y.max(0) as usize;
    let end_x = (x + w).min(WIDTH as i32) as usize;
    let end_y = (y + h).min(HEIGHT as i32) as usize;

    for py in start_y..end_y {
        let row_start = py * (WIDTH as usize) * 4;
        let p_start = row_start + start_x * 4;
        let p_end = row_start + end_x * 4;

        let slice = &mut frame[p_start..p_end];
        for pixel in slice.chunks_exact_mut(4) {
            pixel.copy_from_slice(&color);
        }
    }
}
