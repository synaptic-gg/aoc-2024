use std::collections::btree_map::Keys;

mod input;

fn main() {
    let input = input::data().unwrap();
    let data = parse_data(input);
    println!("{:?}", data);
    let mut count = 0;
    for (head, vec) in data {
        if let Some(k) = check_equation(head, &vec) {
            println!("{} : {:?}", head, k);
            count += head;
        }
    }
    println!("{}", count);
}

fn check_equation(head: i64, numbers: &Vec<i32>) -> Option<Vec<String>> {
    let mut equtaion = numbers
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>();
    let mut sol = vec![];
    gen(numbers, head, &mut equtaion, &mut sol, 0);
    if sol.is_empty() {
        None
    } else {
        Some(sol)
    }
}

fn gen(
    numbers: &Vec<i32>,
    head: i64,
    equtaion: &mut Vec<String>,
    sol: &mut Vec<String>,
    pos: usize,
) {
    let expr = equtaion.join(" ");
    if pos == numbers.len() - 1 {
        if get_value(&expr) == head {
            sol.push(expr);
        }
        return;
    }
    equtaion.insert(2 * pos + 1, "+".to_string());
    gen(numbers, head, equtaion, sol, pos + 1);
    equtaion.remove(2 * pos + 1);

    equtaion.insert(2 * pos + 1, "*".to_string());
    gen(numbers, head, equtaion, sol, pos + 1);
    equtaion.remove(2 * pos + 1);

    equtaion.insert(2 * pos + 1, "||".to_string());
    gen(numbers, head, equtaion, sol, pos + 1);
    equtaion.remove(2 * pos + 1);
}

fn get_value(expr: &str) -> i64 {
    let each: Vec<&str> = expr.split_whitespace().collect();
    let mut result = each[0].parse::<i64>().unwrap();
    let mut index = 1;
    while index < each.len() {
        let op = each[index];
        let val = each[index + 1].parse::<i64>().unwrap();
        if op == "+" {
            result += val;
        } else if op == "*" {
            result *= val;
        } else if op == "||" {
            result = con(result, val);
        }
        index += 2;
    }
    result
}

fn con(left: i64, right: i64) -> i64 {
    let comb = format!("{}{}", left.to_string(), right.to_string());
    comb.parse::<i64>().unwrap()
}

fn parse_data(input: String) -> Vec<(i64, Vec<i32>)> {
    let mut out: Vec<(i64, Vec<i32>)> = vec![];
    for line in input.split("\n") {
        if line.is_empty() {
            continue;
        }
        let mut data = line.split(":");
        let headdata = data.next().unwrap();
        let taildata = data.next().unwrap();
        println!("{}", headdata);
        let head = headdata.trim().parse::<i64>().unwrap();
        let mut new: (i64, Vec<i32>) = (head, vec![]);
        for tail in taildata.split(" ") {
            if tail.is_empty() {
                continue;
            }
            new.1.push(tail.parse::<i32>().unwrap());
        }
        out.push(new);
    }
    out
}
