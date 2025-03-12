use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pair {
    pub x: f32,
    pub y: f32,
}

impl Pair {
    pub fn new(x: f32, y: f32) -> Self {
        Pair { x, y }
    }

    pub fn abs(&self) -> Pair {
        Pair {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn normalize_or_zero(self) -> Self {
        let length = (self.x * self.x + self.y * self.y).sqrt();
        if length > 0.0 {
            Self {
                x: self.x / length,
                y: self.y / length,
            }
        } else {
            Self { x: 0.0, y: 0.0 }
        }
    }

    pub fn dot(self, other: Pair) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn magnitude(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(self) -> Pair {
        let mag = self.magnitude();
        if mag == 0.0 { self } else { self / mag }
    }

    pub fn distance(self, other: Pair) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn angle(self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn rotate(self, angle: f32) -> Pair {
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();
        Pair {
            x: self.x * cos_theta - self.y * sin_theta,
            y: self.x * sin_theta + self.y * cos_theta,
        }
    }

    pub fn cross_prod(self, other: Pair) -> Pair {
        Pair {
            x: self.x * other.y - self.y * other.x,
            y: self.y * other.x - self.x * other.y,
        }
    }
}

impl Add for Pair {
    type Output = Pair;
    fn add(self, other: Pair) -> Pair {
        Pair {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Pair {
    type Output = Pair;
    fn sub(self, other: Pair) -> Pair {
        Pair {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Pair {
    type Output = Pair;
    fn mul(self, scalar: f32) -> Pair {
        Pair {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Mul<Pair> for f32 {
    type Output = Pair;
    fn mul(self, pair: Pair) -> Pair {
        Pair {
            x: pair.x * self,
            y: pair.y * self,
        }
    }
}

impl Div<f32> for Pair {
    type Output = Pair;
    fn div(self, scalar: f32) -> Pair {
        Pair {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl Neg for Pair {
    type Output = Pair;
    fn neg(self) -> Pair {
        Pair {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl AddAssign for Pair {
    fn add_assign(&mut self, other: Pair) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl SubAssign for Pair {
    fn sub_assign(&mut self, other: Pair) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl MulAssign<f32> for Pair {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl DivAssign<f32> for Pair {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.magnitude().partial_cmp(&other.magnitude())
    }
}
