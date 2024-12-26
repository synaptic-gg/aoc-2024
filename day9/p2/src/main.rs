mod input;
fn main() {
    let input = input::data().unwrap();
    let data = input
        .trim()
        .chars()
        .map(|n| (n.to_digit(10).unwrap() as i32))
        .collect::<Vec<i32>>();
    let mut count = 0;
    let mut file: Vec<String> = vec![];
    let mut file_size: Vec<i32> = vec![];
    let mut file_start: Vec<i32> = vec![];
    let mut off_set = 0;
    for (i, v) in data.iter().enumerate() {
        if i % 2 == 0 {
            file_start.push(off_set);
            file_size.push(v.clone());
            for _ in 0..*v {
                file.push(count.to_string());
                off_set += 1
            }
            count += 1;
        } else {
            for _ in 0..*v {
                file.push(".".to_string());
                off_set += 1
            }
        }
    }
    for i in (0..file_start.len()).rev() {
        let current_pos = file_start[i] as usize;
        let size = file_size[i];
        let mut super_fill_start = None;
        let mut fill_start = None;
        let mut fill_free = 0;
        count -= 1;
        for (k, block) in file.iter().enumerate() {
            //println!("{}",count);
            if block == &count.to_string() {
                break;
            }
            if block == "." {
                if fill_start.is_none() {
                    fill_start = Some(k);
                }
                fill_free += 1;

                if fill_free >= size {
                    super_fill_start = fill_start;
                    break;
                }
            } else {
                fill_free = 0;
                fill_start = None;
            }
        }
        if let Some(val) = super_fill_start {
            for i in 0..size {
                let n = i as usize;
                file[val + n] = file[current_pos + n].clone();
                file[current_pos + n] = ".".to_string();
            }
        }
    }

    //println!("{:?} ", file);
    let mut result = 0;
    for (a, b) in file.iter().enumerate() {
        if b != "." {
            let k = b.parse::<i64>().unwrap();
            result += (a as i64) * k;
        }
    }

    println!("{}", result)
}
