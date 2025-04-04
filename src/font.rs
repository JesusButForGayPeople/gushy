use crate::State;
use fontdue::{Font, Metrics};
use std::collections::HashMap;
use tiny_skia::{Color, IntRect, Paint, Pixmap, Transform};

pub struct CachedGlyph {
    pub metrics: Metrics,
    pub bitmap: Vec<u8>,
}

pub fn draw_glyph(
    pixmap: &mut Pixmap,
    cache: &mut HashMap<char, CachedGlyph>,
    font: &Font,
    c: char,
    x: &mut f64,
    y: &mut f64,
    font_size: f64,
) {
    let cached_glyph = cache.entry(c).or_insert_with(|| {
        let (metrics, bitmap) = font.rasterize(c, font_size as f32);
        CachedGlyph { metrics, bitmap }
    });

    let glyph_x = *x + cached_glyph.metrics.xmin as f64;

    let mut glyph_y = *y - cached_glyph.metrics.height as f64;

    if cached_glyph.metrics.ymin < 0 {
        glyph_y -= cached_glyph.metrics.ymin as f64
    }

    for row in 0..cached_glyph.metrics.height {
        for col in 0..cached_glyph.metrics.width {
            let index = row * cached_glyph.metrics.width + col;
            if index < cached_glyph.bitmap.len() {
                let alpha = cached_glyph.bitmap[index];
                if alpha > 0 {
                    let color = Color::from_rgba8(0, 0, 0, alpha);
                    let mut paint = Paint::default();
                    paint.set_color(color);

                    let px = glyph_x + col as f64;
                    let py = glyph_y + row as f64;

                    if px >= 0.0
                        && py >= 0.0
                        && px < pixmap.width() as f64
                        && py < pixmap.height() as f64
                    {
                        let rect = IntRect::from_xywh(px as i32, py as i32, 1, 1);
                        pixmap.fill_rect(
                            rect.unwrap().to_rect(),
                            &paint,
                            Transform::identity(),
                            None,
                        );
                    }
                }
            }
        }
    }

    *x += cached_glyph.metrics.advance_width as f64 + 2.0; // Adjust spacing

    // if *x > pixmap.width() as f64 - 40.0 {
    //     *x = 20.0;
    //     *y += font_size as f64 + 10.0; // Adjust line spacing
    // }
}

pub fn draw_text(
    pixmap: &mut Pixmap,
    cache: &mut HashMap<char, CachedGlyph>,
    font: &Font,
    text: &str,
    start_x: f64,
    start_y: f64,
    font_size: f64,
    align: TextAlign,
) {
    let text_width: f64 = text
        .chars()
        .map(|c| {
            let cached_glyph = cache.entry(c).or_insert_with(|| {
                let (metrics, bitmap) = font.rasterize(c, font_size as f32);
                CachedGlyph { metrics, bitmap }
            });
            cached_glyph.metrics.advance_width as f64
        })
        .sum();

    let x = match align {
        TextAlign::Left => start_x,
        TextAlign::Center => start_x - text_width / 2.0,
        TextAlign::Right => start_x - text_width,
    };

    let mut cursor_x = x;
    let mut cursor_y = start_y;

    for c in text.chars() {
        draw_glyph(
            pixmap,
            cache,
            &font,
            c,
            &mut cursor_x,
            &mut cursor_y,
            font_size,
        );
    }
}

#[allow(dead_code)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}
