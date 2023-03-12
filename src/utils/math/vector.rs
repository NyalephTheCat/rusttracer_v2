use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    pub fn zero() -> Vector {
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn dot(&self, other: &Vector) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&self) -> Vector {
        let length = self.length();
        Vector {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn lerp(&self, other: &Vector, t: f64) -> Vector {
        *self + (*other - *self) * t
    }

    pub fn reflect(&self, normal: &Vector) -> Vector {
        *self - *normal * 2.0 * self.dot(normal)
    }

    pub fn refract(&self, normal: &Vector, etai_over_etat: f64) -> Vector {
        let cos_theta = (-*self).dot(normal).min(1.0);
        let r_out_perp = *self + *normal * cos_theta * etai_over_etat;
        let r_out_parallel = *normal * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
        r_out_perp + r_out_parallel
    }
}

impl From<(f64, f64, f64)> for Vector {
    fn from(tuple: (f64, f64, f64)) -> Vector {
        Vector {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

impl From<Vector> for (f64, f64, f64) {
    fn from(vector: Vector) -> (f64, f64, f64) {
        (vector.x, vector.y, vector.z)
    }
}

impl From<Vector> for [f64; 3] {
    fn from(vector: Vector) -> [f64; 3] {
        [vector.x, vector.y, vector.z]
    }
}

impl From<[f64; 3]> for Vector {
    fn from(array: [f64; 3]) -> Vector {
        Vector {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f64> for Vector {
    type Output = Vector;

    fn add(self, other: f64) -> Vector {
        Vector {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl Add<Vector> for f64 {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self + other.x,
            y: self + other.y,
            z: self + other.z,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Vector) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl AddAssign<f64> for Vector {
    fn add_assign(&mut self, other: f64) {
        self.x += other;
        self.y += other;
        self.z += other;
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<f64> for Vector {
    type Output = Vector;

    fn sub(self, other: f64) -> Vector {
        Vector {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl Sub<Vector> for f64 {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self - other.x,
            y: self - other.y,
            z: self - other.z,
        }
    }
}

impl SubAssign for Vector {
    fn sub_assign(&mut self, other: Vector) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl SubAssign<f64> for Vector {
    fn sub_assign(&mut self, other: f64) {
        self.x -= other;
        self.y -= other;
        self.z -= other;
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Vector {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl MulAssign<f64> for Vector {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, other: f64) -> Vector {
        Vector {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl DivAssign<f64> for Vector {
    fn div_assign(&mut self, other: f64) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
