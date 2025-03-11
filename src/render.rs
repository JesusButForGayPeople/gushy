use crate::State;
use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Rect, Stroke, Transform};

pub fn draw_dots(pixmap: &mut Pixmap, state: &State) {
    let zoom = state.zoom.max(0.1);
    let mut paint = Paint::default();

    for dot in &state.dots {
        let mut pb = PathBuilder::new();
        let radius = (3.0 * zoom) / 5.0;
        let width = pixmap.width();
        let height = pixmap.height();
        let x_offset = width as f32 / 2.0;
        let y_offset = height as f32 / 2.0;

        pb.push_circle(dot.position.x + x_offset, dot.position.y + y_offset, radius);

        let color = Color::from_rgba8(207, 31, 72, 255);
        paint.set_color(color);
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
                Color::from_rgba8(89, 112, 67, 255) // Light color
            } else {
                Color::from_rgba8(119, 163, 77, 255) // Dark color
            };

            let mut paint = Paint::default();
            paint.set_color(color);

            let rect =
                Rect::from_xywh(x as f32, y as f32, tile_size as f32, tile_size as f32).unwrap();
            pixmap.fill_rect(rect, &paint, Transform::identity(), None);
        }
    }

    // Define border thickness (in pixels, converted from inches assuming 96 DPI)
    let border_thickness = 50.0; // Adjust as needed
    let border_color = Color::from_rgba8(0, 0, 0, 255); // Black color
    let mut border_paint = Paint::default();
    border_paint.set_color(border_color);
    let stroke = Stroke {
        width: 5.0,
        ..Default::default()
    };
    // Create border path
    let mut pb = PathBuilder::new();
    let border_rect = Rect::from_xywh(
        border_thickness,
        border_thickness,
        (width as f32 - 2.0 * border_thickness),
        (height as f32 - 2.0 * border_thickness),
    )
    .unwrap();
    pb.push_rect(border_rect);
    let path = pb.finish().unwrap();

    // Stroke the border path
    pixmap.stroke_path(&path, &border_paint, &stroke, Transform::identity(), None);
}
