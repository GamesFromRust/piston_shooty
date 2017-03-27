use std::ops::*;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn magnitude(&self) -> f64 {
        return ((self.x * self.x) + (self.y * self.y) as f64).sqrt();
    }

    pub fn normalize(&mut self) {
        let inv_mag = 1.0 / self.magnitude();
        self.x *= inv_mag;
        self.y *= inv_mag;
    }

    pub fn normalized(&self) -> Vector2 {
        let inv_mag = 1.0 / self.magnitude();
        Vector2 {
            x: self.x * inv_mag,
            y: self.y * inv_mag,
        }
    }

    // pub fn dot(&self, rhs: &Vector2) -> f64 {
    //     return (self.x * rhs.x) + (self.y * rhs.y);
    // }
}

impl fmt::Display for Vector2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {0}\n y: {1}", self.x, self.y)
    }
}

impl Default for Vector2 {
    fn default() -> Vector2 {
        Vector2 { x: 0.0, y: 0.0 }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Vector2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// TODO: How the fuck does the memory work out here? What's copied?
impl Sub for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<f64> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f64) -> Vector2 {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Div<f64> for Vector2 {
    type Output = Vector2;
    fn div(self, rhs: f64) -> Vector2 {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl MulAssign<f64> for Vector2 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
    }
}
