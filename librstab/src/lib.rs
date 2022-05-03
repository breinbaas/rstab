use std::env;
use std::fs::File;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Geometry {
    levee_code: String,
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

        //let contents =
        //    fs::read_to_string(d.as_path()).expect("Something went wrong reading the file");
        assert_eq!(geometry.levee_code, "test");
    }
}
