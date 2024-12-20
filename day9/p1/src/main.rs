use std::vec;

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
    for (i, v) in data.iter().enumerate() {
        if i % 2 == 0 {
            for _ in 0..*v {
                file.push(count.to_string());
            }
            count += 1;
        } else {
            for _ in 0..*v {
                file.push(".".to_string())
            }
        }
    }
    //println!("{:?}",file);
    let mut file_data: Vec<String> = vec![];
    let mut file_free: Vec<i32> = vec![];
    for (i, v) in file.iter().enumerate() {
        if v == "." {
            file_free.push(i as i32);
        } else {
            file_data.push(v.clone());
        }
    }
    //println!("{:?} {:?}",file_free,file_data);
    for i in file_free {
        if file_data.len() - 1 < i as usize {
            break;
        }
        let k = file_data.pop().unwrap();
        file_data.insert(i as usize, k);
    }
    // println!("{:?}", file_data);
    let mut c = 0;
    let mut result = 0;
    for i in file_data {
        let k = i.parse::<i64>().unwrap();
        //println!("{}",k);
        result += k * c;
        c += 1;
    }

    println!("{}", result)
}
