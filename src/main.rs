use pixels::{Error, Pixels, SurfaceTexture};

use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event::VirtualKeyCode::C;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;


use crate::drawable::Drawable;
use crate::primitives::camera::Camera;
use crate::primitives::color::Color;
use crate::primitives::cube::Cube3;
use crate::primitives::cubic_face3::CubicFace3;
use crate::primitives::position::Pose;
use crate::primitives::textures::bw::BWTexture;
use crate::primitives::textures::colored::ColoredTexture;
use crate::primitives::vector::Vector3;

use crate::worlds::World;

mod boxy_world;
mod drawable;
mod primitives;
mod worlds;
mod motion_model;

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

    // Create a world with a standard camera
    let mut world = World::new(Camera::new(
        Pose::new(Vector3::empty(), 0.0),
        100.0, WIDTH as f32 / 2., HEIGHT as f32 / 2.,
    ));

    // Create some 3D objects
    /*
    */

    let c = Color::purple();

    for i in -5..5 {
        for j in -5..5 {
            let bottom_face = CubicFace3::from_line(
                Vector3::new(2.*i as f32, 2.*j as f32, 0.0),
                Vector3::new(2.*i as f32 + 1.0, 2.*j as f32, 0.0),
                false,
                Box::new(ColoredTexture::new(c.randomize_dimension(2))),
            );
            let cube = Cube3::from_face(bottom_face, 2.0, Color::purple());
            world.add_cube(cube);
        }
    }
    //
    // let bottom_face = CubicFace3::from_line(
    //     Vector3::new(3.0, 0.0, 0.0),
    //     Vector3::new(4.0, 0.0, 0.0),
    //     false,
    //     Box::new(ColoredTexture::new(Color::yellow())),
    // );
    // let cube = Cube3::from_face(bottom_face, 2.0, Color::purple());
    // world.add_cube(cube);

    // world.add_face(
    //     CubicFace3::create_simple_face(
    //         1.5,
    //         0.,
    //         2.,
    //         4.,
    //         4.,
    //         Box::new(BWTexture::new(0.5, 0.5))
    //     )
    // );

    // Sets the camera as looking at the object
    world.set_camera_position(Vector3::new(-2.0, 0.0, 0.0));

    // Parse the world as a drawable
    let mut world: Box<dyn Drawable> = Box::new(world);

    let supported_keys_pressed = [
        VirtualKeyCode::R,
        VirtualKeyCode::E,
    ];

    let supported_keys_held = [
        VirtualKeyCode::Down,
        VirtualKeyCode::Up,
        VirtualKeyCode::Left,
        VirtualKeyCode::Right,
        VirtualKeyCode::J,
        VirtualKeyCode::K,
    ];

    // UI loop
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

            // Handle some keys
            for key in supported_keys_pressed {
                if input.key_pressed(key) {
                    world.key_pressed(key)
                }
            }

            for key in supported_keys_held {
                if input.key_held(key) {
                    world.key_held(key)
                }
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
