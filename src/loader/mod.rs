use std::fs;
use std::path::{Path,PathBuf};
use crate::geometry::{Point,Trigon};

fn read_obj_file(dir: &str, name: &str) -> String {
    let mut path_buf: PathBuf = Path::new(dir).join(name);
    path_buf.set_extension("obj");
    fs::read_to_string(path_buf.to_str().unwrap()).unwrap()
}

fn parse_vertex(line: &str) -> Point {
    let data: Vec<&str> = line.split(' ').collect();
    let x: f64 = data[1].parse().unwrap();
    let y: f64 = data[2].parse().unwrap();
    let z: f64 = data[3].parse().unwrap();
    Point::new(x, y, z)
}

fn parse_vertex_lines(lines: Vec<&str>) -> Vec<Point> {
    lines
        .iter()
        .map(|line| parse_vertex(line))
        .collect()
}

fn parse_face_vertex_data(data: &str) -> usize {
    let params: Vec<&str> = data.split('/').collect();
    params[0].parse().unwrap()
}

fn parse_face(line: &str) -> (usize, usize, usize) {
    let data: Vec<&str> = line.split(' ').collect();
    let v1 = parse_face_vertex_data(data[1]);
    let v2 = parse_face_vertex_data(data[2]);
    let v3 = parse_face_vertex_data(data[3]);
    (v1 - 1, v2 - 1, v3 - 1)
}

fn parse_face_lines<'a>(
    lines: Vec<&str>,
    vertices: Vec<Point>
) -> Vec<Trigon<'a>> {
    lines
        .iter()
        .map(|line| {
            let (v1, v2, v3) = parse_face(line);
            let p1 = vertices[v1].clone();
            let p2 = vertices[v2].clone();
            let p3 = vertices[v3].clone();
            Trigon::new(p1, p2, p3)
        })
        .collect()
}

fn parse_obj_data<'a>(data: String) -> Vec<Trigon<'a>> {
    let mut vertex_lines: Vec<&str>  = vec![];
    let mut face_lines: Vec<&str> = vec![];

    for line in data.lines() {
        if line.starts_with("v ") {
            vertex_lines.push(line);
        } else if line.starts_with("f ") {
            face_lines.push(line);
        }
    }

    let vertices = parse_vertex_lines(vertex_lines);
    let faces = parse_face_lines(face_lines, vertices);

    faces
}


pub fn fetch_object<'a>(dir: &str, name: &str) -> Vec<Trigon<'a>> {
    let data: String = read_obj_file(dir, name);
    parse_obj_data(data)
}