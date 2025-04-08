
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut file= OpenOptions::new().append(true).create(true).open(path).expect("failed to open or create file");
    write!(file, "{}", content).expect("failed to write to file");

}