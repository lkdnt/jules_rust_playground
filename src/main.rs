mod core;
mod ecs;
mod input;
mod render;
mod types;

use log::{error, info};
use pixels::{Pixels, SurfaceTexture};
use std::time::Instant;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::core::EngineState;
use crate::types::{HEIGHT, TIME_STEP, WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Starting 2D Engine");

    let event_loop = EventLoop::new()?;
    let window = WindowBuilder::new()
        .with_title("DOD 2D Engine")
        .with_inner_size(LogicalSize::new(WINDOW_WIDTH, WINDOW_HEIGHT))
        .with_min_inner_size(LogicalSize::new(WIDTH, HEIGHT))
        .build(&event_loop)?;

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

    // Create the pixel buffer
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;

    let mut engine = EngineState::new();

    // Game loop state
    let mut last_update_time = Instant::now();
    let mut accumulator = 0.0;

    event_loop.run(move |event, elwt| {
        // Run continuously
        elwt.set_control_flow(ControlFlow::WaitUntil(Instant::now() + std::time::Duration::from_secs_f64(TIME_STEP)));

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    info!("Window closed, exiting");
                    elwt.exit();
                }
                WindowEvent::Resized(size) => {
                    if let Err(err) = pixels.resize_surface(size.width, size.height) {
                        error!("Failed to resize pixels surface: {}", err);
                        elwt.exit();
                    }
                }
                WindowEvent::KeyboardInput { event: key_event, .. } => {
                    let is_pressed = key_event.state.is_pressed();
                    engine.input.handle_key_event(key_event.physical_key, is_pressed);
                }
                WindowEvent::RedrawRequested => {
                    // 1. Clear frame
                    render::clear(pixels.frame_mut(), [30, 30, 30, 255]); // Dark grey background

                    // 2. Render entities
                    render::draw_world(pixels.frame_mut(), &engine.world);

                    // 3. Present to screen
                    if let Err(err) = pixels.render() {
                        error!("pixels.render failed: {}", err);
                        elwt.exit();
                    }
                }
                _ => (),
            },
            Event::AboutToWait => {
                // Fixed Time Step implementation
                let now = Instant::now();
                let dt = now.duration_since(last_update_time).as_secs_f64();
                last_update_time = now;
                accumulator += dt;

                while accumulator >= TIME_STEP {
                    engine.update();
                    accumulator -= TIME_STEP;
                }

                // Request a redraw
                window.request_redraw();
            }
            _ => (),
        }
    })?;

    Ok(())
}
