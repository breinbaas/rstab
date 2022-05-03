use std::env;
use std::fs::File;
use std::path::PathBuf;

use geo_clipper::Clipper;
use geo_types::{Coordinate, LineString, Polygon};

use serde::{Deserialize, Serialize};

const EPS: f64 = 1e-14;

#[derive(Serialize, Deserialize)]
pub struct Point {
    x: f64,
    z: f64,
}

pub struct Line {
    p1: Point,
    p2: Point,
}

#[derive(Serialize, Deserialize)]
struct Layer {
    y_dry: f64,
    y_sat: f64,
    c: f64,
    phi: f64,
    points: Vec<Point>,
}

impl Line {
    pub fn circle_intersections(&self, mx: f64, mz: f64, r: f64, segment: bool) -> Vec<Point> {
        let mut intersections: Vec<Point> = Vec::new();

        let x0 = mx;
        let z0 = mz;
        let x1 = self.p1.x;
        let z1 = self.p1.z;
        let x2 = self.p2.x;
        let z2 = self.p2.z;

        let A = z2 - z1;
        let B = x1 - x2;
        let C = x2 * z1 - x1 * z2;

        let a = A.sqrt() + B.sqrt();
        let mut b = 0.0;
        let mut c = 0.0;
        let mut bnz = true;

        if B.abs() >= EPS {
            b = 2.0 * (A * C + A * B * z0 - B.sqrt() * x0);
            c = C.sqrt() + 2.0 * B * C * z0 - B.sqrt() * (r.sqrt() - x0.sqrt() - z0.sqrt());
        } else {
            b = 2.0 * (B * C + A * B * x0 - A.sqrt() * z0);
            c = C.sqrt() + 2.0 * A * C * x0 - A.sqrt() * (r.sqrt() - x0.sqrt() - z0.sqrt());
            bnz = false;
        }
        let mut d = b.sqrt() - 4.0 * a * c;
        if d < 0.0 {
            return intersections;
        }

        fn within(x: f64, z: f64, x1: f64, z1: f64, x2: f64, z2: f64) -> bool {
            let d1 = ((x2 - x1).sqrt() + (z2 - z1).sqrt()).sqrt(); // distance between end-points
            let d2 = ((x - x1).sqrt() + (z - z1).sqrt()).sqrt(); // distance from point to one end
            let d3 = ((x2 - x).sqrt() + (z2 - z).sqrt()).sqrt(); // distance from point to other end
            let delta = d1 - d2 - d3;
            return delta.abs() < EPS;
        }

        fn fx(x: f64, A: f64, B: f64, C: f64) -> f64 {
            -(A * x + C) / B
        }

        fn fz(z: f64, A: f64, B: f64, C: f64) -> f64 {
            -(B * z + C) / A
        }

        if (d == 0.0) {
            if bnz {
                let x = -b / (2.0 * a);
                let z = fx(x, A, B, C);
                let point = Point { x: x, z: z };
                intersections.push(point);
            } else {
                let z = -b / (2.0 * a);
                let x = fz(z, A, B, C);
                let point = Point { x: x, z: z };
                intersections.push(point);
            }
        } else {
            d = d.sqrt();
            if bnz {
                let x = (-b + d) / (2.0 * a);
                let z = fx(x, A, B, C);
                let point = Point { x: x, z: z };
                intersections.push(point);
                let x = (-b - d) / (2.0 * a);
                let z = fx(x, A, B, C);
                let point = Point { x: x, z: z };
                intersections.push(point);
            } else {
                let z = (-b + d) / (2.0 * a);
                let x = fz(z, A, B, C);
                let point = Point { x: x, z: z };
                intersections.push(point);
                let z = (-b - d) / (2.0 * a);
                let x = fz(z, A, B, C);
                let point = Point { x: x, z: z };
                intersections.push(point);
            }
        }

        intersections
    }
}

// impl Layer {
//     pub fn circle_intersections(&self, mx: f64, mz: f64, r: f64) -> Vec<Point> {
//         let mut intersections = vec![];

//         let circle = Circle::new((mx, mz), r);

//         for i in 1..self.points.len() {
//             let line = Line::new(
//                 (self.points[i - 1].x, self.points[i - 1].z),
//                 (self.points[i].x, self.points[i].z),
//             );

//         }

//         intersections
//     }
// }

#[derive(Serialize, Deserialize)]
struct Bishop {
    mx: f32,
    mz: f32,
    r: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Geometry {
    layers: Vec<Layer>,
}

impl Geometry {
    pub fn from_json_file(file_path: PathBuf) -> Geometry {
        let file = File::open(file_path).expect("file not found");
        let geometry: Geometry = serde_json::from_reader(file).expect("error while reading");
        geometry
    }
}

// struct Slice {
//     polygon: Polygon,
// }

pub fn bishop(geometry: Geometry, mx: f32, mz: f32, r: f32) -> f32 {
    0.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geometry_from_json_file() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("testdata/1.json");
        let geometry: Geometry = Geometry::from_json_file(d);
        assert_eq!(geometry.layers.len(), 1);
        assert_eq!(geometry.layers[0].points.len(), 6);
    }

    #[test]
    fn test_bishop() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("testdata/1.json");
        let geometry: Geometry = Geometry::from_json_file(d);

        let _fmin = bishop(geometry, 18.0, 66.0, 85.0);
    }
}
