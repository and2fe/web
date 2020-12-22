use std::{env, io::Read};

#[derive(Debug)]
struct Error(String);

impl<T: std::error::Error> From<T> for Error {
    fn from(t: T) -> Self {
        Error(t.to_string())
    }
}

fn main() -> Result<(), Error> {
    let fname = env::args()
        .skip(1)
        .next()
        .ok_or(Error("Usage: test <filename>".to_string()))?;
    let mut text = String::new();
    std::fs::File::open(fname)?.read_to_string(&mut text)?;
    Ok(())
}
