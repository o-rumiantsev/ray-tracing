use std::f64::{EPSILON,INFINITY};
use std::mem::swap;

use super::Point;
use super::Vector;
use super::Trigon;
use crate::tree::BoundingBox;

pub fn intersection(vector: &Vector, face: &Trigon) -> f64 {
    let p1 = &face.points[0];
    let p2 = &face.points[1];
    let p3 = &face.points[2];

    let edge1 = Vector::from(p2 - p1);
    let edge2 = Vector::from(p3 - p1);

    let pvec = vector.cross_product(&edge2);
    let det = edge1.dot_product(&pvec);

    if det < EPSILON && det > -EPSILON {
        return INFINITY;
    }

    let tvec = Vector::from(vector.origin - p1);
    let u = tvec.dot_product(&pvec) / det;

    if u < 0. || u > 1. {
        return INFINITY;
    }

    let qvec = tvec.cross_product(&edge1);
    let v = vector.dot_product(&qvec) / det;

    if v < 0. || u + v > 1. {
        return INFINITY;
    }

    edge2.dot_product(&qvec) / det
}

pub fn vector_box_intersection(
    vector: &Vector,
    min: &Point,
    max: &Point,
) -> f64 {
    let (x0, y0, z0) = (min.x, min.y, min.z);
    let (x1, y1, z1) = (max.x, max.y, max.z);

    let normalized = vector.normalize();
    let (m, n, p) = (normalized.x, normalized.y, normalized.z);
    let (x, y, z) = (vector.origin.x, vector.origin.y, vector.origin.z);

    let mut t_near = -INFINITY;
    let mut t_far = INFINITY;

    if m == 0. {
        if x > x1 || x < x0 {
            return INFINITY;
        }
    } else {
        t_near = (x0 - x) / m;
        t_far = (x1 - x) / m;
        if t_near > t_far { swap(&mut t_near, &mut t_far); }
    }

    if n == 0. {
        if y > y1 || y < y0 {
            return INFINITY;
        }
    } else {
        let mut t1y = (y0 - y) / n;
        let mut t2y = (y1 - y) / n;

        if t1y > t2y { swap(&mut t1y, &mut t2y); }

        if t1y > t_near { t_near = t1y; }
        if t2y < t_far { t_far = t2y; }
    }

    if t_near > t_far || t_far < 0. {
        return INFINITY;
    }

    if p == 0. {
        if z > z1 || z < z0 {
            return INFINITY;
        }
    } else {
        let mut t1z = (z0 - z) / p;
        let mut t2z = (z1 - z) / p;

        if t1z > t2z { swap(&mut t1z, &mut t2z); }

        if t1z > t_near { t_near = t1z; }
        if t2z < t_far { t_far = t2z; }
    }

    if t_near > t_far || t_far == 0. {
        return INFINITY;
    }

    t_near
}

pub fn trigon_box_intersection(
    trigon: &Trigon,
    bounding_box: &BoundingBox
) -> bool {
    let c = &bounding_box.center;
    let e = &bounding_box.extents;

    let v0 = Vector::from(&trigon.points[0] - c);
    let v1 = Vector::from(&trigon.points[1] - c);
    let v2 = Vector::from(&trigon.points[2] - c);

    let f0 = &v1 - &v0;
    let f1 = &v2 - &v1;
    let f2 = &v0 - &v2;

    let u0 = Vector::new(1., 0., 0.);
    let u1 = Vector::new(0., 1., 0.);
    let u2 = Vector::new(0., 0., 1.);

    let axis_u0_f0 = u0.cross_product(&f0);
    let axis_u0_f1 = u0.cross_product(&f1);
    let axis_u0_f2 = u0.cross_product(&f2);

    let axis_u1_f0 = u1.cross_product(&f0);
    let axis_u1_f1 = u1.cross_product(&f1);
    let axis_u1_f2 = u1.cross_product(&f2);

    let axis_u2_f0 = u2.cross_product(&f0);
    let axis_u2_f1 = u2.cross_product(&f1);
    let axis_u2_f2 = u2.cross_product(&f2);

    let test = |axis| test_axis(axis, &v0, &v1, &v2, &u0, &u1, &u2, e);

    if !test(&axis_u0_f0) { return false; }
    if !test(&axis_u0_f1) { return false; }
    if !test(&axis_u0_f2) { return false; }

    if !test(&axis_u1_f0) { return false; }
    if !test(&axis_u1_f1) { return false; }
    if !test(&axis_u1_f2) { return false; }

    if !test(&axis_u2_f0) { return false; }
    if !test(&axis_u2_f1) { return false; }
    if !test(&axis_u2_f2) { return false; }

    if !test(&u0) { return false; }
    if !test(&u1) { return false; }
    if !test(&u2) { return false; }

    if !test(&trigon.normal) { return false; }

    true
}

fn test_axis(
    axis: &Vector,
    v0: &Vector,
    v1: &Vector,
    v2: &Vector,
    u0: &Vector,
    u1: &Vector,
    u2: &Vector,
    e: &Point,
) -> bool {
    let p0 = v0.dot_product(axis);
    let p1 = v1.dot_product(axis);
    let p2 = v2.dot_product(axis);

    let r = e.x * u0.dot_product(axis).abs() +
        e.y * u1.dot_product(axis).abs() +
        e.z * u2.dot_product(axis).abs();

    let min = p0.min(p1).min(p2);
    let max = p0.max(p1).max(p2);

    min.max(-max) <= r
}
