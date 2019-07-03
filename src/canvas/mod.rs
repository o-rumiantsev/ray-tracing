use crate::geometry::{Point};

// TODO: use view vector
pub fn build(
    height: u32,
    width: u32,
    y: f64,
    distance: f64,
    fov: f32,
) -> Vec<Vec<Point>> {
    let mut canvas: Vec<Vec<Point>> = vec![];

    let real_width = 2. * (fov.to_radians() / 2.).tan() as f64 * distance;
    let delta = real_width / width as f64;
    let real_height = delta * height as f64;

    let x_start = -real_width / 2.;
    let x_end = x_start + real_width as f64;
    let z_start = real_height / 2.;
    let z_end = z_start - real_height as f64;

    let mut z = z_start;

    while z >= z_end {
        let mut row: Vec<Point> = vec![];
        let mut x = x_start;

        while x <= x_end {
            let point = Point::new(x, y, z);
            row.push(point);
            x = x + delta;
        }

        canvas.push(row);
        z = z - delta;
    }

    canvas
}