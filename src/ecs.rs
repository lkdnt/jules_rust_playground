/// Lightweight ECS-like structure driven by Data-Oriented Design (DOD).
/// Uses a Struct of Arrays (SoA) layout.
/// Entities are merely indices.
/// Components are bitmasks on a signature array.

use crate::types::Vec2;

// Maximum number of entities alive at any time
pub const MAX_ENTITIES: usize = 1000;

// Component type bitmasks
pub const COMPONENT_TRANSFORM: u32 = 1 << 0;
pub const COMPONENT_VELOCITY: u32  = 1 << 1;
pub const COMPONENT_RENDERABLE: u32 = 1 << 2;
pub const COMPONENT_PLAYER: u32    = 1 << 3;

/// Component: A 2D Transform
#[derive(Debug, Clone, Copy, Default)]
pub struct Transform {
    pub position: Vec2,
}

/// Component: A 2D Velocity
#[derive(Debug, Clone, Copy, Default)]
pub struct Velocity {
    pub value: Vec2,
}

/// Component: Renderable data (e.g., solid color block for now)
#[derive(Debug, Clone, Copy, Default)]
pub struct Renderable {
    pub color: [u8; 4], // RGBA
    pub size: Vec2,
}

/// The World stores all component arrays in contiguous memory (SoA).
pub struct World {
    /// Each entity has a signature bitmask that defines which components it possesses.
    /// An entity is "alive" if its signature is not 0.
    pub signatures: [u32; MAX_ENTITIES],

    // Component arrays
    pub transforms: [Transform; MAX_ENTITIES],
    pub velocities: [Velocity; MAX_ENTITIES],
    pub renderables: [Renderable; MAX_ENTITIES],
}

impl World {
    pub fn new() -> Self {
        // Initialize everything with defaults.
        // We use arrays directly for flat, cache-friendly data access.
        Self {
            signatures: [0; MAX_ENTITIES],
            transforms: [Transform::default(); MAX_ENTITIES],
            velocities: [Velocity::default(); MAX_ENTITIES],
            renderables: [Renderable::default(); MAX_ENTITIES],
        }
    }

    /// Finds the first dead entity (signature == 0) and reuses it.
    /// Returns the entity index, or None if the world is full.
    pub fn create_entity(&mut self) -> Option<usize> {
        let entity = self.signatures.iter().position(|&sig| sig == 0)?;
        // Reserve the slot so subsequent calls won't return the same index until it's destroyed.
        self.signatures[entity] = 1 << 31;
        Some(entity)
    }

    /// Destroys an entity by simply clearing its signature.
    /// Since we use SoA, clearing the signature is enough to logically delete the entity.
    #[allow(dead_code)]
    #[inline(always)]
    pub fn destroy_entity(&mut self, entity: usize) {
        if entity < MAX_ENTITIES {
            self.signatures[entity] = 0;
        }
    }
}
