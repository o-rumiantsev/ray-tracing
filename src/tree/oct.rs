use std::f64::INFINITY;
use super::BoundingBox;
use crate::geometry::{Vector,Trigon,intersection};

const COUNT_OF_TRIGONS_IN_NODE: usize = 20;

#[derive(Debug)]
pub struct Octree<'a> {
    pub bounding_box: BoundingBox,
    pub children: Vec<Octree<'a>>,
    pub faces: Vec<&'a Trigon<'a>>,
}

impl<'a> Octree<'a> {
    pub fn new(faces: &'a Vec<Trigon<'a>>) -> Self {
        let bounded_faces: Vec<&Trigon> = faces
            .iter()
            .map(|face| face)
            .collect();
        let bounding_box = BoundingBox::from(&faces);
        return Octree::leaf_or_node(&bounding_box, bounded_faces);
    }

    fn leaf_or_node(
        bounding_box: &BoundingBox,
        bounded_faces: Vec<&'a Trigon<'a>>
    ) -> Self {
        if bounded_faces.len() <= COUNT_OF_TRIGONS_IN_NODE {
            return Octree {
                bounding_box: bounding_box.clone(),
                children: vec![],
                faces: bounded_faces,
            };
        }

        let bounding_subboxes = bounding_box.split();
        let mut children: Vec<Octree> = vec![];

        for subbox in bounding_subboxes.iter() {
            let subbounded_faces = subbox.get_bounded(&bounded_faces);
            if subbounded_faces.len() == 0 { continue; }
            let child = Octree::leaf_or_node(subbox, subbounded_faces);
            children.push(child);
        }

        Octree {
            bounding_box: bounding_box.clone(),
            children,
            faces: vec![],
        }
    }

    fn face_intersection(
        &self,
        vector: &Vector
    ) -> (f64, Option<&'a Trigon<'a>>) {
        let mut min_distance = INFINITY;
        let mut trigon = None;

        for &face in &self.faces {
            let distance = intersection(vector, face);
            if distance < min_distance {
                min_distance = distance;
                trigon = Some(face);
            }
        }

        (min_distance, trigon)
    }

    pub fn intersection(
        &self,
        vector: &Vector
    ) -> (f64, Option<&'a Trigon<'a>>) {
        if !self.bounding_box.intersects(vector) {
            return (INFINITY, None);
        }

        if self.faces.len() > 0 {
            return self.face_intersection(vector);
        } else if self.children.len() > 0 {
            let mut child_intersections: Vec<(f64, Option<&'a Trigon<'a>>)> =
                self.children
                    .iter()
                    .map(|child| child.intersection(vector))
                    .collect();

            child_intersections.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

            return child_intersections.remove(0);
        }

        (INFINITY, None)
    }
}