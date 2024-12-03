mod input;
use regex::Regex;

fn main() {
    let data = input::data().unwrap();
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    //let do_ = Regex::new(r"do\(\)").unwrap();
    //let dont = Regex::new(r"don't\(\)").unwrap();

    let mut result = 0;
    let mut enable = true;
    for i in re.captures_iter(&data) {
        let k = i[0].parse::<String>().unwrap();
        if k == "do()" {
            enable = true;
            continue;
        } else if k == "don't()" {
            enable = false;
            continue;
        }
        if enable {
            let a: i32 = i[1].parse().unwrap();
            let b: i32 = i[2].parse().unwrap();
            result += a * b;
        }
    }

    println!("{}", result);
}
