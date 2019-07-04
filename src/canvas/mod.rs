use crate::geometry::{Point,Vector};

pub fn build(
    width: u32,
    height: u32,
    view_vector: &Vector,
    up_vector: &Vector,
    fov: f32,
) -> Vec<Vec<Point>> {
    let mut canvas: Vec<Vec<Point>> = vec![];

    let origin = view_vector.origin;
    let direction = Point::from(view_vector);

    let offset = &direction + origin;

    let scale = 2. * (fov.to_radians() / 2.).tan() as f64;
    let real_width = scale * view_vector.length;
    let delta = real_width / width as f64;
    let real_height = delta * height as f64;

    let view_vector = view_vector.normalize();
    let up_vector = up_vector.normalize();

    let left_vector = up_vector.cross_product(&view_vector);

    let start = left_vector.multiply(real_width / 2.) + up_vector.multiply(real_height / 2.);
    let end = left_vector.multiply(-real_width / 2.) + up_vector.multiply(-real_height / 2.);

    let delta_x_width = left_vector.x.abs() * (end.x - start.x) / width as f64;
    let delta_x_height = up_vector.x.abs() * (end.x - start.x) / height as f64;

    let delta_y_width = left_vector.y.abs() * (end.y - start.y) / width as f64;
    let delta_y_height = up_vector.y.abs() * (end.y - start.y) / height as f64;

    let delta_z_width = left_vector.z.abs() * (end.z - start.z) / width as f64;
    let delta_z_height = up_vector.z.abs() * (end.z - start.z) / height as f64;

    let start_point = &Point::from(&start) + &offset;

    let x = start_point.x;
    let y = start_point.y;
    let z = start_point.z;

    for h in 0..(height + 1) {
        let h = h as f64;
        let mut row: Vec<Point> = vec![];

        for w in 0..(width + 1) {
            let w = w as f64;

            let px = x + w * delta_x_width + h * delta_x_height;
            let py = y + w * delta_y_width + h * delta_y_height;
            let pz = z + w * delta_z_width + h * delta_z_height;

            row.push(Point::new(px, py, pz));
        }

        canvas.push(row);
    }

    canvas
}