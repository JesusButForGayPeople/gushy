use crate::State;
use std::io::Write;

pub fn print_debug(state: &mut State) {
    let (fps, elapsed) = calculate_fps(state);

    // Move the cursor to the top-left and clear the line
    print!("\r");

    let text = format!(
        "FPS: {:.2}\nUp Time: {:.2} sec\nWindow Size: [width: {:?}, height: {:?}] \nTarget Density: {:.2}\nPressure Multiplier: {:.2}\nSpeed Scale: {:.2}\nForce Scale: {:.2}\nMouse Position: ({:?},{:?})",
        fps,
        elapsed,
        state.window_size.width,
        state.window_size.height,
        state.target_density,
        state.pressure_multiplier,
        state.speed_scale,
        state.force_scale,
        state.mouse_info.scaled_mouse_position.x,
        state.mouse_info.scaled_mouse_position.y,
    );

    if state.time_info.frame_count == 1 {
        // Print all debug info normally on first run
        println!("{}", text);
    } else {
        // Move cursor up to overwrite previous text
        print!("\x1B[{}A", 8); // Move cursor up
        for line in text.lines() {
            print!("\x1B[K{}\r\n", line); // Clear line then print new value
        }
    }

    std::io::stdout().flush().unwrap(); // Ensure output is written immediately
}

fn calculate_fps(state: &State) -> (f32, f32) {
    let elapsed = state.time_info.start.elapsed().as_secs_f32();
    let fps = state.time_info.frame_count as f32 / elapsed;

    (fps, elapsed)
}
