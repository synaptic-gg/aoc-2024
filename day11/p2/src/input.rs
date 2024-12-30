use std::fs::File;
use std::io::prelude::*;

pub fn data() -> std::io::Result<String> {
    let mut file = File::open("input.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
