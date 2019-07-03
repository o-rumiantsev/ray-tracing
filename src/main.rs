mod geometry;
mod loader;
mod tree;
mod canvas;
mod tracing;

use std::env;
use tracing::trace;
use geometry::Point;
use loader::fetch_object;

const OBJECTS_DIR: &str = "data/obj";
const IMAGES_DIR: &str = "data/img";

fn argv() -> Vec<String> {
    let mut argv: Vec<String> = vec![];

    for arg in env::args() {
        argv.push(arg);
    }

    argv
}

fn main() {
    let argv = argv();
    let object_name = &argv[1];
    let width: u32 = argv[2].parse().unwrap();
    let height: u32 = argv[3].parse().unwrap();

    let camera_pos = Point::new(0., -1., 0.);
    let light_pos = Point::new(0., 0., 1.);
    let faces = fetch_object(OBJECTS_DIR, object_name.as_str());

    let img = trace(width, height, &camera_pos, &light_pos, &faces);

    let path = format!("{}/{}.bmp", IMAGES_DIR, object_name);
    img.save(path).expect("Cannot save image");
}
