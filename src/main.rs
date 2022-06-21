use std::error::Error;
use glob::glob;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn print_line_count(path: &str, ext: &str) -> Result<(), Box<dyn Error>> {
    let pattern = [path, "**/*.", ext].join("");
    for entry in glob(pattern.as_str())? {
        match entry {
            Ok(entry) => {
                let input = File::open(entry.as_path())?;
                let buffered = BufReader::new(input);
                let line_count = buffered.lines().count();
                println!("{:?} {}", entry.as_path(), line_count);
            }
            Err(err) => {
                println!("Pattern error {}", err);
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    print_line_count("/Users/am/Documents/dev/rust/", "rs")
}
