mod input;
use regex::Regex;

fn main() {
    let data = input::data().unwrap();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_ = Regex::new(r"do\(\)").unwrap();
    let dont = Regex::new(r"don't\(\)").unwrap();

    let mut result = 0;
    let mut enable = true;
    for i in data.split_inclusive(')') {
        if do_.is_match(i) {
            enable = true;
            continue;
        }
        if dont.is_match(i) {
            enable = false;
            continue;
        }
        if let Some(k) = re.captures(i) {
            if enable {
                let a: i32 = k[1].parse().unwrap();
                let b: i32 = k[2].parse().unwrap();
                result += a * b;
            }
        }
    }

    println!("{}", result);
}
