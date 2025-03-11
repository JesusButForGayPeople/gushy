use rand::Rng;
use std::time::Instant;

pub mod debug;
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

#[derive(Clone, Copy)]
pub struct Dot {
    pub position: Pair,
    pub velocity: Pair,
    pub density: f32,
}

impl Dot {
    pub fn new(position: Pair, velocity: Pair, density: f32) -> Self {
        Self {
            position,
            velocity,
            density,
        }
    }
    pub fn position(&self) -> Pair {
        self.position
    }
    pub fn velocity(self) -> Pair {
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
}

impl State {
    pub fn new(window_width: u32, window_height: u32) -> State {
        let dots = generate_dots(window_width as f32, window_height as f32);
        State {
            dots,
            zoom: 20.0,
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
            target_density: 0.5,
            pressure_multiplier: 10.0,
            speed_scale: 1.0,
            force_scale: 1.0,
        }
    }
}

pub fn generate_dots(width: f32, height: f32) -> Vec<Dot> {
    let mut rng = rand::thread_rng();

    (0..500)
        .map(|_| {
            Dot::new(
                Pair::new(
                    rng.gen_range((-width / 2.0) + 60.0..(width / 2.0) - 60.0),
                    rng.gen_range((-height / 2.0) + 60.0..(height / 2.0) - 60.0),
                ),
                Pair::new(rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0)),
                0.0,
            )
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
    let mass = 1.0;
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
    let mass = 1.0;
    let mut total_pressure_force = Pair::new(0.0, 0.0);

    for particle in dots.iter() {
        let distance = (particle.position - center).magnitude();
        if distance <= radius && distance > 0.0 {
            let direction = -(particle.position - center) / distance;
            let slope = derivative_smoothing_kernel(radius, distance);
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

pub fn update_dots(state: &mut State) {
    let gravity = Pair::new(0.0, 0.00);
    let dots_copy = state.dots.clone(); // Avoid borrowing conflicts

    // Compute densities first
    let densities: Vec<f32> = compute_densities(&dots_copy, 10.0);

    // Apply new densities
    for (dot, new_density) in state.dots.iter_mut().zip(densities) {
        dot.density = new_density;
    }

    // Compute pressure forces and update positions
    for dot in state.dots.iter_mut() {
        let pressure_force = calculate_pressure(
            &dots_copy,
            dot.position,
            10.0,
            state.target_density,
            state.pressure_multiplier,
        );
        dot.velocity += (pressure_force + gravity) * state.force_scale;
        dot.position += dot.velocity * state.speed_scale;

        // Boundary conditions
        let dampening_factor = 0.9;
        let boundary_x = (800.0 / 2.0) - 60.0;
        let boundary_y = (600.0 / 2.0) - 60.0;

        if dot.position.x >= boundary_x {
            dot.position.x = boundary_x - 0.01; // Move slightly away
            dot.velocity.x = -dot.velocity.x * dampening_factor;
        } else if dot.position.x <= -boundary_x {
            dot.position.x = -boundary_x + 0.01;
            dot.velocity.x = -dot.velocity.x * dampening_factor;
        }

        if dot.position.y >= boundary_y {
            dot.position.y = boundary_y - 0.01;
            dot.velocity.y = -dot.velocity.y * dampening_factor;
        } else if dot.position.y <= -boundary_y {
            dot.position.y = -boundary_y + 0.01;
            dot.velocity.y = -dot.velocity.y * dampening_factor;
        }
    }
}

pub fn density_to_pressure(density: f32, target_density: f32, pressure_multiplier: f32) -> f32 {
    let density_error = density - target_density;
    density_error * pressure_multiplier
}
