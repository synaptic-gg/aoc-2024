mod input;
use std::collections::HashMap;
fn main() {
    let data = input::data().unwrap();
    let k = 75;
    let op_input = op_parse(data);
    let mut op_blinks = op_input;
    println!("{:?}", op_blinks);
    for _ in 0..k {
        //blinks = blink_magic(blinks);
        op_blinks = op_blink(op_blinks);
    }
    //println!("{:?}",op_blinks);
    //println!("{:?}",blinks);
    let mut result = 0;
    for i in op_blinks {
        result += i.1;
    }

    println!("after {}  stones {}", k, result);
}

fn op_blink(input: HashMap<String, i64>) -> HashMap<String, i64> {
    let mut out = HashMap::new();
    for (i, c) in input.iter() {
        if i == "0" {
            let key = "1".to_string();
            let k = out.get(&key).unwrap_or(&0);
            out.insert(key, *k + *c);
        } else if i.len() % 2 == 0 {
            let (a, b) = i.split_at(i.len() / 2);
            let b: i64 = b.parse().unwrap();
            let key = a.to_string();
            let k = out.get(&key).unwrap_or(&0);
            out.insert(key, *k + *c);
            let key = b.to_string();
            let k = out.get(&key).unwrap_or(&0);
            out.insert(key, *k + *c);
        } else {
            let k: i64 = i.parse().unwrap();
            let key = (k * 2024).to_string();
            let k = out.get(&key).unwrap_or(&0);
            out.insert(key, *k + *c);
        }
    }

    out
}
fn op_parse(data: String) -> HashMap<String, i64> {
    let mut out = HashMap::new();
    for i in data.trim().split(" ") {
        let k = i.trim().to_string();
        let p = out.get(&k).unwrap_or(&0);
        out.insert(k, *p + 1);
    }

    out
}
