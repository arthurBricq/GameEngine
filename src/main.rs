use std::f32::consts::PI;
use std::time::Instant;

use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use crate::drawable::Drawable;
use crate::fps::FPSMonitor;
use crate::frame::Frame;
use crate::primitives::camera::Camera;
use crate::primitives::cube::Cube3;
use crate::primitives::cubic_face3::CubicFace3;
use crate::primitives::textures::bw::BWTexture;
use crate::primitives::textures::colored::{ColoredTexture, ORANGE, PURPLE, YELLOW};
use crate::primitives::textures::pixelated::Pixelated;
use crate::primitives::vector::{UNIT_Z, Vector3};
use crate::worlds::World;

pub mod bsp;
mod drawable;
mod fps;
mod frame;
mod motion_model;
mod png_saver;
mod primitives;
mod worlds;

// For different screen resolution: https://en.wikipedia.org/wiki/Display_resolution

pub const WIDTH: u32 = 480;
pub const HEIGHT: u32 = 320;

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

    let supported_keys_pressed = [VirtualKeyCode::R, VirtualKeyCode::E];

    let supported_keys_held = [
        VirtualKeyCode::Down,
        VirtualKeyCode::Up,
        VirtualKeyCode::Left,
        VirtualKeyCode::Right,
        VirtualKeyCode::J,
        VirtualKeyCode::K,
    ];

    // Texture library is created in the main
    let bw_texture = Box::leak(Box::new(BWTexture::new(0.5, 0.5)));

    // Minecraft texture library
    let soil_side = Box::leak(Box::new(Pixelated::soil_side()));
    let soil_top = Box::leak(Box::new(Pixelated::soil_top()));
    let wood = Box::leak(Box::new(Pixelated::wood()));
    let floor = Box::leak(Box::new(Pixelated::wood_floor()));
    let stone = Box::leak(Box::new(Pixelated::stone()));

    // Create a world with a standard camera
    let mut world = World::new(Camera::default());

    // Create many cubes arranged as a sort of maze
    // let c = Color::purple();
    // let n = 6;
    // for i in -n..n {
    //     for j in -n..n {
    //         let bottom_face = CubicFace3::hface_from_line(
    //             Vector3::new(3.*i as f32, 3.*j as f32, 0.0),
    //             Vector3::new(3.*i as f32 + 1.0, 3.*j as f32, 0.0),
    //         );
    //         let cube = Cube3::from_face(bottom_face, 2.0, &PURPLE);
    //         world.add_cube(cube);
    //     }
    // }

    // ### Create a cube
    // let bottom_face = CubicFace3::hface_from_line(
    //     Vector3::new(0.0, 0.0, 0.0),
    //     Vector3::new(1.0, 0.0, 0.0),
    // );
    // let cube = Cube3::from_face(bottom_face, 2.0, soil_top);
    // world.add_cube(cube);

    // ### Create some faces
    // let mut f1 = CubicFace3::vface_from_line(Vector3::newi2(0, 0), Vector3::newi2(1, 0));
    // let mut f2 = CubicFace3::vface_from_line(Vector3::newi2(2, 0), Vector3::newi2(3, 0));
    // let mut f3 = CubicFace3::vface_from_line(Vector3::newi2(1, 1), Vector3::newi2(2, 1));
    // f1.set_texture(bw_texture);
    // f2.set_texture(pixelated2);
    // f3.set_texture(soil_side);
    // world.add_face(f1);
    // world.add_face(f2);
    // world.add_face(f3);

    // Minecraft blocks
    world.add_cube(Cube3::minecraft_like(Vector3::newi(0,0,0), soil_side, soil_top));
    world.add_cube(Cube3::minecraft_like(Vector3::newi(1,0,0), soil_side, soil_top));
    world.add_cube(Cube3::minecraft_like(Vector3::newi(2,0,0), soil_side, soil_top));
    world.add_cube(Cube3::minecraft_like(Vector3::newi(3,0,0), soil_side, soil_top));
    world.add_cube(Cube3::minecraft_like(Vector3::newi(0,-1,0), wood, wood));
    world.add_cube(Cube3::minecraft_like(Vector3::newi(0,-3,0), stone, stone));
    world.add_cube(Cube3::minecraft_like(Vector3::newi(1,-3,0), stone, stone));

    // Set the floor
    // let x0  = -5;
    // let y0  = -5;
    // let x1 = 5;
    // let y1 = 5;
    // world.add_face(CubicFace3::new(
    //     [Vector3::newi(x0, y0, 1),Vector3::newi(x1, y0, 1),Vector3::newi(x1, y1, 1),Vector3::newi(x0, y1, 1)],
    //     UNIT_Z.opposite(),
    //     floor,
    // ));

    // Sets the camera as looking at the object
    world.set_camera_position(Vector3::new(2., -3., -1.5));
    world.set_camera_rotation(-PI / 2.);

    // Benchmarking the maze
    // world.set_camera_position(Vector3::new(0.11243102, -23.725393, -6.0802684));
    // world.set_camera_rotation(-PI / 2.);

    // Calling this function will (i) build the BSP tree and (ii) force the renderer to use it
    // when using the painter algorithm.
    // world.compute_bsp();

    // Run the main loop
    let mut fps_monitor = FPSMonitor::new();
    let mut use_fps_monitor = false;
    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            // Draw the background color
            let background = [214, 214, 194, 150];
            for pixel in pixels.frame_mut().chunks_exact_mut(4) {
                pixel.copy_from_slice(&background);
            }

            // For using painter algorithm (with or without binary space partitioning)
            let mut current_frame = Frame::new(pixels.frame_mut());
            world.draw_painter(&mut current_frame);

            // For using raytracing algorithn:
            // world.draw_raytracing(pixels.frame_mut());

            if let Err(err) = pixels.render() {
                log_error("pixels.render", err);
                *control_flow = ControlFlow::Exit;
                return;
            }

            fps_monitor.add_frame(Instant::now());

            if use_fps_monitor {
                fps_monitor.log_fps();
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

            // Handle some keys to be sent to the world
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

            // Debug options
            if input.key_pressed(VirtualKeyCode::F1) {
                use_fps_monitor = !use_fps_monitor;
                println!("Using FPS monitor = {use_fps_monitor}");
            } else if input.key_pressed(VirtualKeyCode::F2) {
                println!("Cam position = {:?}", world.camera().pose().position());
                println!(
                    "Cam orientation = {:?}",
                    world.camera().pose().orientation()
                );
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
