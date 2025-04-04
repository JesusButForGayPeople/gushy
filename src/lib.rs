use crate::font::CachedGlyph;
use fontdue::Font;
use rand::Rng;
use std::collections::HashMap;
use std::time::Instant;
use tiny_skia::Color;

pub mod debug;
pub mod font;
pub mod math;
pub mod render;
use crate::math::Pair;

pub const CURSOR_RADIUS: f32 = 50.0;

pub struct TimeInfo {
    pub last_frame_time: Instant,
    pub frame_count: u64, // Frame count for debugging
    pub start: Instant,
    pub delta_time: f32,
}

pub struct MouseInfo {
    pub mouse_down: bool,
    pub mouse_position: Pair,
    pub mouse_position_last: Option<Pair>,
    pub mouse_delta: Pair,
    pub scaled_mouse_position: Pair,
}

#[derive(Debug, Clone, Copy)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}
impl WindowSize {
    pub fn new(width: u32, height: u32) -> WindowSize {
        WindowSize { width, height }
    }
}

#[derive(Clone)]
pub struct Dot {
    pub position: Pair,
    pub velocity: Pair,
    pub density: f32,
    pub color: Color,
    pub distance_to_cursor: f32,
    pub is_selected: bool,
    pub label: String,
}

impl Dot {
    pub fn new(position: Pair, velocity: Pair, density: f32, color: Color) -> Self {
        Self {
            position,
            velocity,
            density,
            color,
            distance_to_cursor: 0.0,
            is_selected: false,
            label: String::from("A File Eventually..."),
        }
    }
    pub fn position(&self) -> Pair {
        self.position
    }
    pub fn velocity(&self) -> Pair {
        self.velocity
    }
}

pub struct State {
    pub dots: Vec<Dot>, // Points for the animation
    pub zoom: f32,
    pub window_size: WindowSize,
    pub time_info: TimeInfo,
    pub mouse_info: MouseInfo,
    pub target_density: f32,
    pub pressure_multiplier: f32,
    pub speed_scale: f32,
    pub force_scale: f32,
    pub focus_color: Option<Color>,
    pub font: Font,
    pub glyph_cache: HashMap<char, CachedGlyph>,
}

impl State {
    pub fn new(ndots: usize, window_width: u32, window_height: u32) -> State {
        let dots = generate_dots(ndots, window_width as f32, window_height as f32, 150.0);
        let font_data = include_bytes!("../fonts/LTInternet-Regular.ttf") as &[u8];
        let font = Font::from_bytes(font_data, fontdue::FontSettings::default()).unwrap();
        State {
            dots,
            zoom: 40.0,
            mouse_info: MouseInfo {
                mouse_down: false,
                mouse_position: Pair::new(0.0, 0.0),
                mouse_position_last: None,
                mouse_delta: Pair::new(0.0, 0.0),
                scaled_mouse_position: Pair::new(0.0, 0.0),
            },
            time_info: TimeInfo {
                last_frame_time: Instant::now(),
                frame_count: 0,
                start: Instant::now(),
                delta_time: 0.0,
            },
            window_size: WindowSize::new(window_width, window_height),
            target_density: 0.05,
            pressure_multiplier: 10.0,
            speed_scale: 1.0 / ndots as f32,
            force_scale: 1.0 / ndots as f32,
            focus_color: None,
            font,
            glyph_cache: HashMap::new(),
        }
    }
}

pub fn generate_dots(ndots: usize, width: f32, height: f32, orbit_radius: f32) -> Vec<Dot> {
    let mut rng = rand::thread_rng();
    let center = Pair::new(0.0, 0.0);
    let speed = (orbit_radius / (ndots as f32 * 10.0)).sqrt() * 0.1; // Adjust the speed for stable orbit

    (0..ndots)
        .map(|_| {
            let angle = rng.gen_range(1.0..3.0 * std::f32::consts::PI);
            let position = Pair::new(
                center.x + orbit_radius * angle.cos(),
                center.y + orbit_radius * angle.sin(),
            );
            let r: u8 = (207 + rng.gen_range(-30..30)) as u8;
            let g: u8 = (31 + rng.gen_range(-30..30)) as u8;
            let b: u8 = (72 + rng.gen_range(-30..30)) as u8;
            let velocity = Pair::new(-speed / 2.0 * angle.sin(), speed / 2.5 * angle.cos());
            Dot::new(position, velocity, 0.0, Color::from_rgba8(r, g, b, 255))
        })
        .collect()
}

pub fn distance(p1: Pair, p2: Pair) -> f32 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}

pub fn smoothing_kernel(radius: f32, distance: f32) -> f32 {
    let volume = std::f32::consts::PI * radius.powi(8) / 4.0;
    let smoothing = (radius * radius - distance * distance).max(0.0);
    smoothing * smoothing * smoothing / volume
}

pub fn derivative_smoothing_kernel(radius: f32, distance: f32) -> f32 {
    let f = radius * radius - distance * distance;
    let scale = -24.0 / (std::f32::consts::PI * radius.powi(8));
    scale * distance * f * f
}

pub fn compute_densities(dots: &[Dot], radius: f32) -> Vec<f32> {
    let mass = 1.0 / 2000.0;
    dots.iter()
        .map(|dot_i| {
            dots.iter()
                .filter(|dot_j| distance(dot_i.position, dot_j.position) <= radius)
                .map(|dot_j| {
                    mass * smoothing_kernel(radius, distance(dot_i.position, dot_j.position))
                })
                .sum::<f32>() // Sum all nearby mass contributions
        })
        .collect()
}

pub fn calculate_pressure(
    dots: &[Dot],
    center: Pair,
    radius: f32,
    target_density: f32,
    pressure_multiplier: f32,
) -> Pair {
    let mass = 1.0 / 2000.0;
    let mut total_pressure_force = Pair::new(0.0, 0.0);

    for particle in dots.iter() {
        let distance = (particle.position - center).magnitude().abs();
        if distance <= radius {
            let direction = -(particle.position - center) / distance.max(0.0001);
            let slope = derivative_smoothing_kernel(radius, distance.max(0.0001));
            let density = particle.density;

            if density != 0.0 {
                let pressure_force =
                    density_to_pressure(density, target_density, pressure_multiplier)
                        * direction
                        * slope
                        * mass
                        / density;

                total_pressure_force += pressure_force;
            }
        }
    }

    total_pressure_force
}

pub const CENTER_REPULSIVE_RADIUS: f32 = 200.0; //
pub const PARTICLE_REPULSIVE_RADIUS: f32 = 150.0; //

pub fn update_dots(state: &mut State) {
    let gravity = Pair::new(0.0, 0.00);
    let dots_copy = state.dots.clone(); // Avoid borrowing conflicts
    // Compute densities first
    let densities: Vec<f32> = compute_densities(&dots_copy, 10.0);

    // Apply new densities
    for (dot, new_density) in state.dots.iter_mut().zip(densities) {
        dot.density = new_density;
    }

    // Define the center of the screen
    let center = Pair::new(0.0, 0.0);
    let circular_force_strength = 0.75; // Adjust the strength of the circular force
    let repulsive_force_strength = 0.75; // Adjust the strength of the repulsive force

    // Compute pressure forces and update positions
    for dot in state.dots.iter_mut() {
        let pressure_force = calculate_pressure(
            &dots_copy,
            dot.position,
            10.0,
            state.target_density,
            state.pressure_multiplier,
        );

        // Calculate the direction to the center
        let to_center = center - dot.position;
        let distance_to_center = to_center.magnitude().abs();
        let direction_to_center = to_center / distance_to_center.max(0.0001);

        // Calculate the required centripetal force for circular motion
        let centripetal_force_magnitude = dot.velocity.magnitude().powi(2) / distance_to_center;
        let centripetal_force =
            direction_to_center * centripetal_force_magnitude * circular_force_strength;

        // Apply the centripetal force
        dot.velocity +=
            (pressure_force + gravity + ((45.0 / dots_copy.len() as f32) * centripetal_force))
                * state.force_scale;

        // Apply a repulsive force near the center to prevent dots from getting stuck
        if distance_to_center < CENTER_REPULSIVE_RADIUS {
            let repulsive_force = -direction_to_center * repulsive_force_strength;
            dot.velocity += repulsive_force + Pair::new(-0.02, 0.2);
        }

        for other_dot in &dots_copy {
            if dot.position != other_dot.position {
                let to_other = other_dot.position - dot.position;
                let distance_to_other = to_other.magnitude().abs();
                if distance_to_other < PARTICLE_REPULSIVE_RADIUS + ((12.0 * state.zoom) / 5.0) {
                    let direction_to_other = to_other / distance_to_other.max(0.0001);
                    let repulsive_force =
                        -direction_to_other * repulsive_force_strength / distance_to_other;
                    dot.velocity += repulsive_force;
                }
            }
        }

        dot.position += dot.velocity * state.speed_scale;

        // Boundary conditions
        let dampening_factor = 0.85;
        let boundary_x = (800.0 / 2.0) - 40.0;
        let boundary_y = (600.0 / 2.0) - 40.0;

        if dot.position.x >= boundary_x {
            dot.position.x = boundary_x - 0.5; // Move slightly away
            dot.velocity.x = -dot.velocity.x * dampening_factor;
            dot.velocity.y *= dampening_factor; // Reduce y velocity to avoid getting stuck in corners
        } else if dot.position.x <= -boundary_x {
            dot.position.x = -boundary_x + 0.5;
            dot.velocity.x = -dot.velocity.x * dampening_factor;
            dot.velocity.y *= dampening_factor; // Reduce y velocity to avoid getting stuck in corners
        }

        if dot.position.y >= boundary_y {
            dot.position.y = boundary_y - 0.5;
            dot.velocity.y = -dot.velocity.y * dampening_factor;
            dot.velocity.x *= dampening_factor; // Reduce x velocity to avoid getting stuck in corners
        } else if dot.position.y <= -boundary_y {
            dot.position.y = -boundary_y + 0.5;
            dot.velocity.y = -dot.velocity.y * dampening_factor;
            dot.velocity.x *= dampening_factor; // Reduce x velocity to avoid getting stuck in corners
        }
    }
}

pub fn density_to_pressure(density: f32, target_density: f32, pressure_multiplier: f32) -> f32 {
    let density_error = density - target_density;
    density_error * pressure_multiplier
}
