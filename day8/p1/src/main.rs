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
    let mut sol: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut count = vec![];
    let mut keys = 0;
    for (val, x, y) in towers_pos {
        towers.entry(val).or_insert_with(Vec::new).push((x, y));
    }
    println!("{:?}", towers);
    for (tower, pos) in towers {
        keys +=1;
        for i in 0..pos.len() {
            for j in i + 1..pos.len() {
                let mut d_x = pos[i].0 as i32 - pos[j].0 as i32;
                let mut d_y = pos[i].1 as i32 - pos[j].1 as i32;
               
                let (a, b) = (pos[i].0 as i32 + d_x, pos[i].1 as i32 + d_y);
                let (c, d) = (pos[j].0 as i32 - d_x, pos[j].1 as i32 - d_y);

                if a > 0 && a < x_len && b > 0 && b < x_len {
                   sol.entry(tower).or_insert_with(Vec::new).push((a, b));
                    if !count.contains(&(a, b)) {
                        count.push((a, b));
                       
                    }

                    println!("{:?}", (a, b));
                }
                if c > 0 && c < x_len && d > 0 && d < x_len {
                    sol.entry(tower).or_insert_with(Vec::new).push((c, d));
                    if !count.contains(&(c, d)) {
                        count.push((c, d));

                    }

                    println!("{:?} {} {}", (c, d),d_y,d_x);
                }

            }
        }
    }
    println!("{:?} \n {}", sol, count.len());
    

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
