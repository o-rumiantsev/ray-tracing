use crate::tree::Octree;
use super::{Point,Vector,Trigon};

pub fn distance(p1: &Point, p2: &Point) -> f64 {
    let sqr_delta_x = (p2.x - p1.x).powi(2);
    let sqr_delta_y = (p2.y - p1.y).powi(2);
    let sqr_delta_z = (p2.z - p1.z).powi(2);
    (sqr_delta_x + sqr_delta_y + sqr_delta_z).sqrt()
}

pub fn trigon_brightness(
    light_pos: &Point,
    trigon: &Trigon,
    tree: &Octree,
) -> u8 {
    let mut light_vector = Vector::from(&trigon.centroid - light_pos);
    light_vector.set_origin(&light_pos);

    let (distance, obstacle) = tree.intersection(&light_vector);

    if distance < std::f64::INFINITY && obstacle.unwrap() != trigon {
        return 0;
    }

    let (a, b, c) = (
        trigon.normal.x,
        trigon.normal.y,
        trigon.normal.z,
    );
    let (m, n, p) = (
        light_vector.x,
        light_vector.y,
        light_vector.z,
    );

    let len1 = trigon.normal.length;
    let len2 = light_vector.length;

    let cos = (a * m + b * n + c * p).abs() / (len1 * len2);

    (cos * 255.).floor() as u8
}