mod input;
use regex::Regex;
fn main() {
    // let  data = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let data = input::data().unwrap();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut result = 0;
    for i in re.captures_iter(&data) {
        let a: i32 = i[1].parse().unwrap();
        let b: i32 = i[2].parse().unwrap();
        result += a * b;
    }
    println!("{}", result)
}
