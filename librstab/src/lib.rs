use std::env;
use std::fs::File;
use std::path::PathBuf;

use geo_clipper::Clipper;
use geo_types::{Coordinate, LineString, Polygon};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Point {
    x: f32,
    z: f32,
}

#[derive(Serialize, Deserialize)]
struct Layer {
    y_dry: f32,
    y_sat: f32,
    c: f32,
    phi: f32,
    points: Vec<Point>,
}

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
