use std::cmp::PartialEq;
use super::point::Point;
use super::vector::Vector;

#[derive(Debug,Clone)]
pub struct Trigon<'a> {
    pub points: Vec<Point>,
    pub normal: Vector<'a>,
    pub centroid: Point,
}

impl<'a> Trigon<'a> {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Self {
        let edge1 = Vector::from(&p2 - &p1);
        let edge2 = Vector::from(&p3 - &p1);

        let normal = edge1.cross_product(&edge2);
        let centroid = Point::new(
            (p1.x + p2.x + p3.x) / 3.,
            (p1.y + p2.y + p3.y) / 3.,
            (p1.z + p2.z + p3.z) / 3.
        );

        Trigon {
            points: vec![p1, p2, p3],
            normal,
            centroid,
        }
    }
}

impl<'a> PartialEq for Trigon<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.points[0] == other.points[0] &&
            self.points[1] == other.points[1] &&
            self.points[2] == other.points[2]
    }
}