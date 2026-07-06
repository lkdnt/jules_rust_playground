/// Core game logic and fixed update loop implementation

use crate::ecs::{
    World, COMPONENT_PLAYER, COMPONENT_RENDERABLE, COMPONENT_TRANSFORM, COMPONENT_VELOCITY,
};
use crate::input::{InputState, KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_UP};
use crate::types::Vec2;

pub struct EngineState {
    pub world: World,
    pub input: InputState,
}

impl EngineState {
    pub fn new() -> Self {
        let mut world = World::new();

        // Spawn a test player entity
        if let Some(player) = world.create_entity() {
            world.signatures[player] = COMPONENT_TRANSFORM
                | COMPONENT_VELOCITY
                | COMPONENT_RENDERABLE
                | COMPONENT_PLAYER;

            world.transforms[player].position = Vec2::new(200.0, 150.0);
            world.renderables[player].size = Vec2::new(16.0, 16.0);
            world.renderables[player].color = [255, 0, 0, 255]; // Red square
        }

        Self {
            world,
            input: InputState::new(),
        }
    }

    /// Fixed time step update.
    pub fn update(&mut self) {
        // Prepare input state for this tick
        self.input.update();

        // 1. Process Input (Player control)
        let player_mask = COMPONENT_PLAYER | COMPONENT_VELOCITY;
        let speed = 100.0 * crate::types::TIME_STEP as f32; // Pixels per second scaled by fixed time step

        for i in 0..self.world.signatures.len() {
            if (self.world.signatures[i] & player_mask) == player_mask {
                let mut vel = Vec2::new(0.0, 0.0);

                // Branchless velocity calculation driven by boolean values mapped to float
                let left = self.input.is_held(KEY_LEFT) as i32 as f32;
                let right = self.input.is_held(KEY_RIGHT) as i32 as f32;
                let up = self.input.is_held(KEY_UP) as i32 as f32;
                let down = self.input.is_held(KEY_DOWN) as i32 as f32;

                vel.x = (right - left) * speed;
                vel.y = (down - up) * speed;

                self.world.velocities[i].value = vel;
            }
        }

        // 2. Physics / Movement Integration
        let move_mask = COMPONENT_TRANSFORM | COMPONENT_VELOCITY;
        for i in 0..self.world.signatures.len() {
            if (self.world.signatures[i] & move_mask) == move_mask {
                let vel = self.world.velocities[i].value;
                self.world.transforms[i].position.x += vel.x;
                self.world.transforms[i].position.y += vel.y;
            }
        }
    }
}
