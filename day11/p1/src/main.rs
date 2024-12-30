mod input;
fn main() {
    let data = input::data().unwrap();
    let input = parse(data);
    println!("{:?}", input);
    let k = 25;
    let mut blinks = input;
    for _ in 0..k {
        blinks = blink_magic(blinks);
        //println!("{:?}", blinks)
    }
    println!("after {}  stones {}", k, blinks.len())
}

fn blink_magic(input: Vec<String>) -> Vec<String> {
    let mut out: Vec<String> = vec![];
    for i in input {
        if i == "0" {
            out.push("1".to_string());
        } else if i.len() % 2 == 0 {
            let (a, b) = i.split_at(i.len() / 2);
            let k: i64 = b.parse().unwrap();
            out.push(a.to_string());
            out.push(k.to_string());
        } else {
            let k: i64 = i.parse().unwrap();
            out.push((k * 2024).to_string());
        }
    }
    out
}

fn parse(data: String) -> Vec<String> {
    let out = data
        .trim()
        .split(" ")
        .map(|a| a.trim().to_string())
        .collect::<Vec<String>>();
    out
}
