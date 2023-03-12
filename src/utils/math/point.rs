use std::ops::{Add, Div, Mul, Neg, Sub};

use super::vector::Vector;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }

    pub fn zero() -> Point {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn distance(&self, other: &Point) -> f64 {
        let d = *self - *other;
        d.length()
    }

    pub fn lerp(&self, other: &Point, t: f64) -> Point {
        *self + (*other - *self) * t
    }
}

impl From<Vector> for Point {
    fn from(vector: Vector) -> Point {
        Point {
            x: vector.x,
            y: vector.y,
            z: vector.z,
        }
    }
}

impl From<Point> for Vector {
    fn from(point: Point) -> Vector {
        Vector {
            x: point.x,
            y: point.y,
            z: point.z,
        }
    }
}

impl From<(f64, f64, f64)> for Point {
    fn from(tuple: (f64, f64, f64)) -> Point {
        Point {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

impl From<Point> for (f64, f64, f64) {
    fn from(point: Point) -> (f64, f64, f64) {
        (point.x, point.y, point.z)
    }
}

impl From<[f64; 3]> for Point {
    fn from(array: [f64; 3]) -> Point {
        Point {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }
}

impl From<Point> for [f64; 3] {
    fn from(point: Point) -> [f64; 3] {
        [point.x, point.y, point.z]
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f64> for Point {
    type Output = Point;

    fn add(self, other: f64) -> Point {
        Point {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, other: Vector) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, other: Point) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, other: Vector) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, other: f64) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, other: f64) -> Point {
        Point {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
