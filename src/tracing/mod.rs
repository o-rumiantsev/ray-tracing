extern crate bmp;

use std::f64::INFINITY;
use bmp::{Image,Pixel,consts as colors};
use crate::geometry::{Point,Vector,Trigon,utils::trigon_brightness};
use crate::tree::Octree;
use crate::canvas;

pub fn trace(
    width: u32,
    height: u32,
    camera_pos: &Point,
    light_pos: &Point,
    faces: &Vec<Trigon>
) -> Image {
    let tree = Octree::new(&faces);

    let mut img = Image::new(width, height);
    let pixels = canvas::build(height, width,-0.5,0.5,90.);

    for (x, y) in img.coordinates() {
        let pixel = &pixels[y as usize][x as usize];
        let mut vector = Vector::from(pixel - camera_pos);
        vector.set_origin(camera_pos);

        let (distance, trigon) = tree.intersection(&vector);

        if distance < INFINITY {
            let brightness = trigon_brightness(
                light_pos,
                &trigon.unwrap(),
                &tree,
            );
            img.set_pixel(
                x,
                y,
                Pixel::new(brightness, brightness, brightness)
            );
        } else {
            img.set_pixel(x, y, colors::WHITE);
        }
    }

    img
}
