use pixels::{Pixels, SurfaceTexture};
use tiny_skia::Pixmap;
use tokio::runtime::Runtime;
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, MouseButton, MouseScrollDelta, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use gushy::{debug::print_debug, math::*, render::*, *};

fn main() {
    // Initialize the Tokio runtime
    let runtime = Runtime::new().unwrap();
    runtime.block_on(async {
        // Create an event loop
        let event_loop = EventLoop::new();

        // Create a window

        let window = WindowBuilder::new()
            .with_title("tiny-skia Animation")
            .with_inner_size(LogicalSize::new(800.0, 600.0))
            .build(&event_loop)
            .unwrap();

        // Get window size
        let window_size = window.inner_size();

        // Create a pixel buffer
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

        let mut pixels =
            Pixels::new(window_size.width, window_size.height, surface_texture).unwrap();

        // Create the animation state
        let mut state = State::new(window_size.width, window_size.height);

        let mut background_cache = Pixmap::new(window_size.width, window_size.height)
            .expect("Failed to create background cache");
        draw_background(&mut background_cache, &state);
        // Run the event loop
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let Some(keycode) = input.virtual_keycode {
                            match (keycode, input.state) {
                                (VirtualKeyCode::Q, ElementState::Pressed) => {
                                    *control_flow = ControlFlow::Exit
                                }
                                (VirtualKeyCode::Up, ElementState::Pressed) => {
                                    state.pressure_multiplier += 1.0;
                                }
                                (VirtualKeyCode::Down, ElementState::Pressed) => {
                                    state.pressure_multiplier -= 1.0;
                                }
                                (VirtualKeyCode::Left, ElementState::Pressed) => {
                                    state.target_density -= 10.0;
                                }
                                (VirtualKeyCode::Right, ElementState::Pressed) => {
                                    state.target_density += 10.0;
                                }
                                _ => {}
                            }
                        }
                    }
                    WindowEvent::MouseWheel {
                        delta: MouseScrollDelta::LineDelta(_, y),
                        ..
                    } => {
                        let zoom_factor = 1.0 + (y * 0.05);
                        state.zoom = (state.zoom * zoom_factor).clamp(0.1, 50.0);
                    }
                    WindowEvent::MouseWheel { .. } => {} // Handle other MouseScrollDelta variants
                    WindowEvent::MouseInput {
                        button,
                        state: mouse_state,
                        ..
                    } => {
                        if button == MouseButton::Left {
                            match mouse_state {
                                ElementState::Pressed => {
                                    if !state.mouse_info.mouse_down {
                                        state.mouse_info.mouse_down = true;
                                        state.mouse_info.mouse_position_last = None;
                                    }
                                }
                                ElementState::Released => state.mouse_info.mouse_down = false,
                            }
                        }
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        state.mouse_info.mouse_position =
                            Pair::new(position.x as f32, position.y as f32);

                        // Calculate the scaled mouse position
                        let adjusted_x = state.mouse_info.mouse_position.x
                            * (800.0 / (state.window_size.width as f32));
                        let adjusted_y = state.mouse_info.mouse_position.y
                            * (600.0 / (state.window_size.height as f32));

                        state.mouse_info.scaled_mouse_position = Pair::new(adjusted_x, adjusted_y);

                        if state.mouse_info.mouse_down {
                            if let Some(last_position) = &state.mouse_info.mouse_position_last {
                                let delta = state.mouse_info.mouse_position - *last_position;
                                state.mouse_info.mouse_delta = delta;
                            }
                            state.mouse_info.mouse_position_last =
                                Some(state.mouse_info.mouse_position);
                        }
                    }
                    _ => {}
                },
                Event::RedrawRequested(_) => {
                    let mut pixmap = Pixmap::new(window_size.width, window_size.height).unwrap();

                    state.window_size =
                        WindowSize::new(window.inner_size().width, window.inner_size().height);
                    draw_background(&mut pixmap, &state);
                    draw_dots(&mut pixmap, &state);

                    let frame = pixels.get_frame_mut();

                    frame.copy_from_slice(pixmap.data());

                    if let Err(_err) = pixels.render() {
                        *control_flow = ControlFlow::Exit;
                    }
                }
                Event::MainEventsCleared => {
                    state.time_info.frame_count += 1;
                    update_dots(&mut state);

                    print_debug(&mut state);
                    window.request_redraw();
                }
                _ => {}
            }
        });
    });
}
