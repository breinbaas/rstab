use std::env;
use std::fs::File;
use std::path::PathBuf;

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
struct Geometry {
    layers: Vec<Layer>,
}

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }

    #[test]
    fn read_file() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("testdata/1.json");
        let file = File::open(d).expect("file not found");
        let geometry: Geometry = serde_json::from_reader(file).expect("error while reading");
        assert_eq!(geometry.layers.len(), 1);
        assert_eq!(geometry.layers[0].points.len(), 4);
    }
}
