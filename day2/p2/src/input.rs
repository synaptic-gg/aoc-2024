use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
pub fn data() -> std::io::Result<HashMap<String, Vec<i32>>> {
    let mut data = HashMap::new();
    let mut file = File::open("input_san.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    for line in content.lines() {
        if let Some((key, value)) = line.split_once('=') {
            let key = key.trim();
            let value = value
                .trim()
                .trim_matches(|n| n == '[' || n == ']')
                .split(',')
                .filter_map(|n| n.trim().parse::<i32>().ok())
                .collect::<Vec<_>>();
            data.insert(key.to_string(), value);
        }
    }

    Ok(data)
}
