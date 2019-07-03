use std::ops::{Add,Sub};
use super::utils::distance;
use super::point::{Point,START};

#[derive(Debug,Clone)]
pub struct Vector<'a> {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub origin: &'a Point,
    pub length: f64,
}

impl<'a> Vector<'a> {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        let origin = &START;
        let length = distance(&origin, &Point::new(x, y, z));
        Vector {
            x,
            y,
            z,
            origin,
            length,
        }
    }

    pub fn from(direction: Point) -> Self {
        let origin = &START;
        let length = distance(&origin, &direction);
        Vector {
            x: direction.x,
            y: direction.y,
            z: direction.z,
            origin,
            length,
        }
    }

    pub fn set_origin(&mut self, origin: &'a Point) {
        self.origin = origin;
    }

    pub fn multiply(&self, n: f64) -> Self {
        let x = self.x * n;
        let y = self.y * n;
        let z = self.z * n;
        Vector::new(x, y, z)
    }

    pub fn normalize(&self) -> Vector {
        let x = self.x / self.length;
        let y = self.y / self.length;
        let z = self.z / self.length;
        Vector::new(x, y, z)
    }

    pub fn cross_product<'b>(&self, other: &'a Vector) -> Vector<'b> {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;
        Vector::new(x, y, z)
    }

    pub fn dot_product(&self, other: &'a Vector) -> f64 {
        let x = self.x * other.x;
        let y = self.y * other.y;
        let z = self.z * other.z;
        x + y + z
    }
}

impl<'a> Add for Vector<'a> {
    type Output = Self;

    fn add(self, other: Vector<'a>) -> Self {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        Vector::new(x, y, z)
    }
}

impl<'a, 'b> Add<&'b Vector<'b>> for &'a Vector<'a> {
    type Output = Vector<'a>;

    fn add(self, other: &'b Vector<'b>) -> Vector<'a> {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;
        Vector::new(x, y, z)
    }
}

impl<'a> Sub for Vector<'a> {
    type Output = Self;

    fn sub(self, other: Vector<'a>) -> Self {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        Vector::new(x, y, z)
    }
}

impl<'a, 'b> Sub<&'b Vector<'b>> for &'a Vector<'a> {
    type Output = Vector<'a>;

    fn sub(self, other: &'b Vector<'b>) -> Vector<'a> {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        Vector::new(x, y, z)
    }
}
