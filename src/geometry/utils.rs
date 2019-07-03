use super::{Point,Vector,Trigon};

pub fn distance(p1: &Point, p2: &Point) -> f64 {
    let sqr_delta_x = (p2.x - p1.x).powi(2);
    let sqr_delta_y = (p2.y - p1.y).powi(2);
    let sqr_delta_z = (p2.z - p1.z).powi(2);
    (sqr_delta_x + sqr_delta_y + sqr_delta_z).sqrt()
}

pub fn trigon_brightness(light_pos: &Point, trigon: &Trigon) -> u8 {
    let light_vector = Vector::from(&trigon.centroid - light_pos);

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

    let cos = (a * m + b * n + c * p) / (len1 * len2) - 1.;

    (cos.abs() * 128.).floor() as u8
}