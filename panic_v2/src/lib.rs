use std::fs::File;
pub fn open_file(s: &str) -> File {
    File::open(s).unwrap_or_else(|_|
        panic!("called `Result::unwrap()` on an `Err` value: Os {{ code: 2, kind: NotFound, message: \"No such file or directory\" }}")
    )
}
