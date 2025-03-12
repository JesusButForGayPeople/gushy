use crate::State;
use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Rect, Stroke, Transform};

pub fn draw_dots(pixmap: &mut Pixmap, state: &State) {
    let zoom = state.zoom.max(0.1);
    let mut paint = Paint::default();
    let min_mouse_distance = state
        .dots
        .iter()
        .map(|dot| dot.distance_to_cursor)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0.01);

    for dot in &state.dots {
        let mut pb = PathBuilder::new();
        let radius = (3.0 * zoom) / 5.0;
        let width = pixmap.width();
        let height = pixmap.height();
        let x_offset = width as f32 / 2.0;
        let y_offset = height as f32 / 2.0;

        pb.push_circle(dot.position.x + x_offset, dot.position.y + y_offset, radius);

        if dot.distance_to_cursor == min_mouse_distance && min_mouse_distance <= 30.0 {
            if dot.is_selected {
                let color = Color::from_rgba8(107, 231, 72, 255);
                paint.set_color(color);
            } else {
                let color = Color::from_rgba8(157, 181, 72, 255);
                paint.set_color(color);
            }
        } else {
            paint.set_color(dot.color);
        }

        if let Some(path) = pb.finish() {
            pixmap.fill_path(
                &path,
                &paint,
                FillRule::Winding,
                Transform::identity(),
                None,
            );
        } else {
            println!(
                "Failed to create path! \ncenter: x={:?}+{:?},     y={:?}+{:?} \nradius: {:?}",
                dot.position.x, x_offset, dot.position.y, y_offset, radius
            );
        }
    }
}

pub fn draw_background(pixmap: &mut Pixmap, state: &State) {
    let width = pixmap.width();
    let height = pixmap.height();

    let tile_size: u32 = 30; // Scale tile size with zoom

    let parallax_factor = 0.8 / state.zoom; // Background moves slower with zoom-in effect
    let x_offset = ((width as f32 / 2.0) * parallax_factor) as u32;
    let y_offset = ((height as f32 / 2.0) * parallax_factor) as u32;

    for y in (0..height).step_by(tile_size as usize) {
        for x in (0..width).step_by(tile_size as usize) {
            let x_adj = x + x_offset;
            let y_adj = y + y_offset;
            let color = if (x_adj / tile_size + y_adj / tile_size) % 2 == 0 {
                Color::from_rgba8(69, 92, 47, 255) // Light color
            } else {
                Color::from_rgba8(99, 143, 57, 255) // Dark color
            };

            let mut paint = Paint::default();
            paint.set_color(color);

            let rect =
                Rect::from_xywh(x as f32, y as f32, tile_size as f32, tile_size as f32).unwrap();
            pixmap.fill_rect(rect, &paint, Transform::identity(), None);
        }
    }

    // Define border thickness (in pixels, converted from inches assuming 96 DPI)
    let border_thickness = 30.0; // Adjust as needed
    let border_color = Color::from_rgba8(157, 181, 72, 255); // Black color
    let mut border_paint = Paint::default();
    border_paint.set_color(border_color);
    let stroke = Stroke {
        width: 5.0,
        ..Default::default()
    };

    // Create border path
    let mut pb = PathBuilder::new();
    pb.push_circle(width as f32 / 2.0, height as f32 / 2.0, 175.0);
    let path = pb.finish().unwrap();

    if let Some(focus_color) = state.focus_color {
        let mut fill_paint = Paint::default();
        fill_paint.set_color(focus_color);
        pixmap.fill_path(
            &path,
            &fill_paint,
            FillRule::Winding,
            Transform::identity(),
            None,
        );
    }

    // Stroke the border path
    pixmap.stroke_path(&path, &border_paint, &stroke, Transform::identity(), None);
}
