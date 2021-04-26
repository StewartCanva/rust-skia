use skia_safe::{Typeface, Data, Font};
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::env::current_dir;

fn main() {
    println!("current dir: {:?}", current_dir());
    let data = data_from_file_path(Path::new("skia-safe/examples/metrics-extractor/EvolveSans-Bold.woff2"));
    let typeface = Typeface::from_data(data, None).unwrap();
    let font = Font::new(typeface, 1.0);
    let (_, metrics) = font.metrics();

    println!("metrics.ascent: {}", metrics.ascent);
    println!("metrics.descent: {}", metrics.descent);
}

pub fn data_from_file_path(file_path: &Path) -> Data {
    let mut file = File::open(file_path).unwrap();
    let mut bytes = vec![];
    file.read_to_end(&mut bytes).unwrap();
    Data::new_copy(&bytes.as_slice())
}