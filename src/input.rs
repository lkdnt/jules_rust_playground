/// Input handling using bitmasks
/// By representing input state as bits in a u32 (or u64), we can:
/// 1. Update states branchlessly (using bitwise OR/AND NOT).
/// 2. Check input states branchlessly (bitwise AND).
/// 3. Detect edges (just pressed, just released) using XOR.

use winit::keyboard::{KeyCode, PhysicalKey};

// Input bitmasks
pub const KEY_UP: u32    = 1 << 0;
pub const KEY_DOWN: u32  = 1 << 1;
pub const KEY_LEFT: u32  = 1 << 2;
pub const KEY_RIGHT: u32 = 1 << 3;
pub const KEY_ACTION: u32 = 1 << 4;

#[derive(Default)]
pub struct InputState {
    /// The state of the inputs in the current frame
    pub current: u32,
    /// The state of the inputs in the previous frame
    pub previous: u32,
}

impl InputState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Called at the start of a fixed update tick
    pub fn update(&mut self) {
        self.previous = self.current;
    }

    /// Map winit virtual key codes to our internal bitmask
    const fn map_key(keycode: KeyCode) -> u32 {
        match keycode {
            KeyCode::ArrowUp | KeyCode::KeyW => KEY_UP,
            KeyCode::ArrowDown | KeyCode::KeyS => KEY_DOWN,
            KeyCode::ArrowLeft | KeyCode::KeyA => KEY_LEFT,
            KeyCode::ArrowRight | KeyCode::KeyD => KEY_RIGHT,
            KeyCode::Space | KeyCode::Enter => KEY_ACTION,
            _ => 0,
        }
    }

    /// Updates the bitmask based on a key press or release.
    /// Branchless operation to set or clear the flag.
    pub fn handle_key_event(&mut self, physical_key: PhysicalKey, is_pressed: bool) {
        if let PhysicalKey::Code(code) = physical_key {
            let mask = Self::map_key(code);
            // Branchless state update:
            // If is_pressed is true, we want to OR the mask.
            // If is_pressed is false, we want to AND NOT the mask.
            // We can do this without if-else by multiplying the mask by is_pressed as u32.
            // However, in Rust, a simple match or bitwise operation with a boolean is clean.
            // A more standard branchless way:
            let pressed_mask = (is_pressed as u32).wrapping_neg() & mask; // all 1s if true, 0s if false
            let cleared_mask = (!is_pressed as u32).wrapping_neg() & mask;

            // Set bits
            self.current |= pressed_mask;
            // Clear bits
            self.current &= !cleared_mask;
        }
    }

    /// Check if a key is currently held down
    #[inline(always)]
    pub fn is_held(&self, mask: u32) -> bool {
        (self.current & mask) != 0
    }

    /// Check if a key was just pressed this frame
    #[allow(dead_code)]
    #[inline(always)]
    pub fn is_pressed(&self, mask: u32) -> bool {
        let changed = self.current ^ self.previous;
        let pressed_now = self.current & mask;
        (changed & pressed_now) != 0
    }

    /// Check if a key was just released this frame
    #[allow(dead_code)]
    #[inline(always)]
    pub fn is_released(&self, mask: u32) -> bool {
        let changed = self.current ^ self.previous;
        let released_now = self.previous & mask;
        (changed & released_now) != 0
    }
}
