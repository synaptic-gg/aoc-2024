mod input;
use std::collections::HashMap;
fn main() {
    let input = input::data().unwrap();
    let data = parse_input(input);
    let x_len = data.len() as i32 - 1;
    let y_len = data[0].len() as i32 - 1;
    println!("{} {}", x_len, y_len);
    println!("{:?}", data);
    let mut towers_pos: Vec<(char, usize, usize)> = vec![];
    for (x, val_x) in data.iter().enumerate() {
        for (y, &val_y) in val_x.iter().enumerate() {
            if val_y != '.' && val_y != '#' && val_y != '/' {
                towers_pos.push((val_y, y, x));
            }
        }
    }

    let mut towers: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut count = vec![];
    for (val, x, y) in towers_pos {
        towers.entry(val).or_insert_with(Vec::new).push((x, y));
    }
    println!("{:?}", towers);
    for (_tower, pos) in towers {
        for i in 0..pos.len() {
            for j in i + 1..pos.len() {
                let par = get_pos(pos[i], pos[j], x_len, y_len, _tower);
                for p in par {
                    count.push(p);
                }
            }
        }
    }
    let mut sol = vec![];
    for i in count {
        if !sol.contains(&(i.0, i.1)) {
            sol.push((i.0, i.1));
        }
    }
    println!("{}", sol.len())
}

fn get_pos(
    p1: (usize, usize),
    p2: (usize, usize),
    x_len: i32,
    y_len: i32,
    to: char,
) -> Vec<(i32, i32, char)> {
    let mut out = vec![];
    let dx = p1.0 as i32 - p2.0 as i32;
    let dy = p1.1 as i32 - p2.1 as i32;
    let mut p1count = 0;
    loop {
        let t1 = p1.0 as i32 + dx * p1count;
        let t2 = p1.1 as i32 + dy * p1count;
        if check((t1, t2), x_len, y_len) {
            out.push((t1, t2, to));
            p1count += 1;
        } else {
            break;
        }
    }
    let mut p2count = 0;
    loop {
        let t1 = p2.0 as i32 - dx * p2count;
        let t2 = p2.1 as i32 - dy * p2count;
        if check((t1, t2), x_len, y_len) {
            out.push((t1, t2, to));
            p2count += 1;
        } else {
            break;
        }
    }

    out
}

fn check(p: (i32, i32), x_len: i32, y_len: i32) -> bool {
    0 < p.0 && p.0 < x_len && 0 < p.1 && p.1 < y_len
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    let mut parsed = vec![];

    for line in input.split("\n") {
        if !line.trim().is_empty() {
            let mut im = line.chars().collect::<Vec<char>>();
            im.push('/');
            im.insert(0, '/');
            parsed.push(im);
        }
    }
    let length = parsed[0].len();
    let im2: Vec<char> = vec!['/'; length];
    parsed.push(im2.clone());
    parsed.insert(0, im2);
    parsed
}
