use std::f64::INFINITY;
use crate::geometry::{
    Point,
    Vector,
    Trigon,
    vector_box_intersection,
    trigon_box_intersection,
};

#[derive(Debug,Clone)]
pub struct BoundingBox {
    pub min: Point,
    pub max: Point,
    pub center: Point,
    pub extents: Point,
}

impl BoundingBox {
    pub fn new(min: Point, max: Point) -> Self {
        let center = Point::new(
            (max.x + min.x) / 2.,
            (max.y + min.y) / 2.,
            (max.z + min.z) / 2.,
        );

        let extents = &max - &center;

        BoundingBox { min, max, center, extents }
    }

    pub fn from(faces: &Vec<Trigon>) -> Self {
        let mut x_min: f64 = 0.;
        let mut x_max: f64 = 0.;
        let mut y_min: f64 = 0.;
        let mut y_max: f64 = 0.;
        let mut z_min: f64 = 0.;
        let mut z_max: f64 = 0.;

        for face in faces {
            for point in &face.points {
                if point.x < x_min {
                    x_min = point.x;
                } else if point.x > x_max {
                    x_max = point.x
                }

                if point.y < y_min {
                    y_min = point.y;
                } else if point.y > y_max {
                    y_max = point.y
                }

                if point.z < z_min {
                    z_min = point.z;
                } else if point.z > z_max {
                    z_max = point.z
                }
            }
        }

        let min = Point::new(x_min, y_min, z_min);
        let max = Point::new(x_max, y_max, z_max);

        BoundingBox::new(min, max)
    }

    pub fn get_bounded<'a>(
        &self,
        faces: &Vec<&'a Trigon<'a>>
    ) -> Vec<&'a Trigon<'a>> {
        let mut bounded: Vec<&Trigon> = vec![];

        for face in faces {
            let includes = trigon_box_intersection(face, self);
            if includes {
                bounded.push(face);
            }
        }

        bounded
    }

    pub fn split(&self) -> [Self; 8] {
        let mid_x = (self.max.x + self.min.x) / 2.;
        let mid_y = (self.max.y + self.min.y) / 2.;
        let mid_z = (self.max.z + self.min.z) / 2.;

        let b1_min = Point::new(self.min.x, self.min.y, self.min.z);
        let b1_max = Point::new(mid_x, mid_y, mid_z);

        let b2_min = Point::new(self.min.x, mid_y, self.min.z);
        let b2_max = Point::new(mid_x, self.max.y, mid_z);

        let b3_min = Point::new(mid_x, self.min.y, self.min.z);
        let b3_max = Point::new(self.max.x, mid_y, mid_z);

        let b4_min = Point::new(mid_x, mid_y, self.min.z);
        let b4_max = Point::new(self.max.x, self.max.y, mid_z);

        let b5_min = Point::new(self.min.x, self.min.y, mid_z);
        let b5_max = Point::new(mid_x, mid_y, self.max.z);

        let b6_min = Point::new(self.min.x, mid_y, mid_z);
        let b6_max = Point::new(mid_x, self.max.y, self.max.z);

        let b7_min = Point::new(mid_x, self.min.y, mid_z);
        let b7_max = Point::new(self.max.x, mid_y, self.max.z);

        let b8_min = Point::new(mid_x, mid_y, mid_z);
        let b8_max = Point::new(self.max.x, self.max.y, self.max.z);

        [
            BoundingBox::new(b1_min, b1_max),
            BoundingBox::new(b2_min, b2_max),
            BoundingBox::new(b3_min, b3_max),
            BoundingBox::new(b4_min, b4_max),
            BoundingBox::new(b5_min, b5_max),
            BoundingBox::new(b6_min, b6_max),
            BoundingBox::new(b7_min, b7_max),
            BoundingBox::new(b8_min, b8_max),
        ]
    }

    pub fn intersects(&self, vector: &Vector) -> bool {
        vector_box_intersection(vector, &self.min, &self.max) < INFINITY
    }
}
