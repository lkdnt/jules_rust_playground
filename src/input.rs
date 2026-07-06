/// Input handling using bitmasks
/// By representing input state as bits in a u64, we can:
/// 1. Update states branchlessly (using bitwise OR/AND NOT).
/// 2. Check input states branchlessly (bitwise AND).
/// 3. Detect edges (just pressed, just released) using XOR.
/// Supports up to 64 custom input actions.

use winit::keyboard::{KeyCode, PhysicalKey};

// Core default bitmasks for the engine
pub const KEY_UP: u64    = 1 << 0;
pub const KEY_DOWN: u64  = 1 << 1;
pub const KEY_LEFT: u64  = 1 << 2;
pub const KEY_RIGHT: u64 = 1 << 3;
pub const KEY_ACTION: u64 = 1 << 4;

/// A simple array-backed map to bind KeyCodes to our action bitmasks.
/// We use an array where the index is the integer representation of the KeyCode.
/// KeyCode is a non-exhaustive enum, but for common keys we can map them efficiently.
/// Since KeyCode isn't just an integer enum from 0..N, we will use a small lookup array
/// for performance, mapping common known keys.
const MAX_KEYCODES: usize = 256;

pub struct InputState {
    /// The state of the inputs in the current frame
    pub current: u64,
    /// The state of the inputs in the previous frame
    pub previous: u64,
    /// Mapping of KeyCode (cast to usize) to an action bitmask
    /// This allows dynamic remapping of keys to actions.
    key_map: [u64; MAX_KEYCODES],
    /// Track physical key states independently to correctly aggregate actions
    /// mapped to multiple keys.
    key_states: [bool; MAX_KEYCODES],
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            current: 0,
            previous: 0,
            key_map: [0; MAX_KEYCODES],
            key_states: [false; MAX_KEYCODES],
        }
    }
}

impl InputState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Maps a `winit::keyboard::KeyCode` to an action bitmask.
    /// Multiple keys can map to the same action (e.g., 'W' and 'UpArrow' to `KEY_UP`).
    pub fn bind_key(&mut self, keycode: KeyCode, action_mask: u64) {
        let idx = keycode as usize;
        if idx < MAX_KEYCODES {
            self.key_map[idx] |= action_mask;
        }
    }

    /// Removes all bindings for a given key.
    #[allow(dead_code)]
    pub fn unbind_key(&mut self, keycode: KeyCode) {
        let idx = keycode as usize;
        if idx < MAX_KEYCODES {
            self.key_map[idx] = 0;
        }
    }

    /// Called at the start of a fixed update tick
    pub fn update(&mut self) {
        self.previous = self.current;
    }

    /// Get the action bitmask for a given keycode.
    #[allow(dead_code)]
    #[inline(always)]
    fn get_mask(&self, keycode: KeyCode) -> u64 {
        let idx = keycode as usize;
        if idx < MAX_KEYCODES {
            self.key_map[idx]
        } else {
            0
        }
    }

    /// Updates the bitmask based on a key press or release.
    pub fn handle_key_event(&mut self, physical_key: PhysicalKey, is_pressed: bool) {
        if let PhysicalKey::Code(code) = physical_key {
            let idx = code as usize;
            if idx < MAX_KEYCODES {
                self.key_states[idx] = is_pressed;
            }

            // Re-aggregate current action state branchlessly from all physical keys
            let mut new_current = 0;
            for i in 0..MAX_KEYCODES {
                // If key is pressed, OR its mask into the new current state.
                let state_mask = (self.key_states[i] as u64).wrapping_neg();
                new_current |= self.key_map[i] & state_mask;
            }
            self.current = new_current;
        }
    }

    /// Check if an action is currently held down
    #[inline(always)]
    pub fn is_held(&self, mask: u64) -> bool {
        (self.current & mask) != 0
    }

    /// Check if an action was just pressed this frame
    #[allow(dead_code)]
    #[inline(always)]
    pub fn is_pressed(&self, mask: u64) -> bool {
        let changed = self.current ^ self.previous;
        let pressed_now = self.current & mask;
        (changed & pressed_now) != 0
    }

    /// Check if an action was just released this frame
    #[allow(dead_code)]
    #[inline(always)]
    pub fn is_released(&self, mask: u64) -> bool {
        let changed = self.current ^ self.previous;
        let released_now = self.previous & mask;
        (changed & released_now) != 0
    }
}
