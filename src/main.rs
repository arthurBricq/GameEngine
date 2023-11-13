
use pixels::{Error, Pixels, SurfaceTexture};
use pixels::wgpu::TextureViewDimension::Cube;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use crate::boxy_world::BoxyWorld;


use crate::drawable::Drawable;
use crate::primitives::cube::Cube3;
use crate::primitives::cubic_face::CubicFace3;
use crate::primitives::position::Position;
use crate::primitives::vector::Vector3;

use crate::worlds::World;

mod boxy_world;
mod drawable;
mod primitives;
mod worlds;

pub const WIDTH: u32 = 320;
pub const HEIGHT: u32 = 240;

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    /// Create a world with 1 object
    let mut world = World::new();
    let bottom_face = CubicFace3::from_line(
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(1.0, 0.0, 0.0),
        false
    );
    let cube = Cube3::from_face(bottom_face, 2.0);
    world.add_object(cube);

    /// Sets the camera as looking at the object
    world.set_camera(Position::new(
        Vector3::new(-2.0, 0.0, 0.0),
        Vector3::new(1.0, 0.0, 0.0)
    ));

    // Parse the world as drawable and start the UI loop
    let mut world: Box<dyn Drawable> = Box::new(world);
    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.frame_mut());
            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // left mouse pressed
            if input.mouse_pressed(0) {
                if let Some(mouse) = input.mouse() {
                    world.left_mouse_pressed(mouse.0 as i16, mouse.1 as i16)
                }
            }

            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    log_error("pixels.resize_surface", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            // Update internal state and request a redraw
            world.update();
            window.request_redraw();
        }
    });
}

fn log_error<E: std::error::Error + 'static>(method_name: &str, err: E) {
    println!("{method_name}() failed: {err}");
}
