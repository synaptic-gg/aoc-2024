mod input;
use std::{collections::HashMap};

fn main() {
    let in_data = input::data().unwrap();
    let mut data = in_data.split("\n\n");
    let con = data.next().unwrap();
    let check_data = data.next().unwrap();
    let hash_con = con_hash(con);
    let pages = check_data.split("\n").into_iter();
    let mut result = 0;
    println!("{:?}", hash_con);
    for page in pages {
        let checked = check(page, hash_con.clone());
        if checked {
            let data = page
                .split(",")
                .filter_map(|n| n.trim().parse::<i32>().ok())
                .collect::<Vec<i32>>();

            let y = data.len() / 2;
            println!("{:?}", data);
            if data.len() != 0 {
                result += data[y]
            }
        }
    }

    println!("{}", result)
    
}

fn sort(condition: HashMap<i32, Vec<i32>>,page:Vec<i32>)-> Vec<i32>{
    let mut out: Vec<i32> = vec![];
    let mut marked: Vec<i32> = vec![];
   let condition = condition;
   let page = page;
fn dfs_post(condition: HashMap<i32, Vec<i32>>,pag:i32){
    for w pag{
        if marked.contains(w){
            continue;
        }else {
            dfs_post(condition, w);
            marked.push(w)
        }}}
dfs_post(condition, page[1]);    
    
println!("{:?}",out);
out

}

fn con_hash(con_data: &str) -> HashMap<i32, Vec<i32>> {
    let mut out: HashMap<i32, Vec<i32>> = HashMap::new();
    let con = con_data.split("\n").into_iter();
    for i in con {
        let mut val = i
            .split("|")
            .filter_map(|n| Some(n.trim().parse::<i32>().unwrap()));
        let i2 = val.next().unwrap();
        //println!("{}",i1);
        let i1 = val.next().unwrap();
        //println!("{}",i2);
        if out.contains_key(&i1) {
            let mut list = out.get(&i1).unwrap().clone();
            list.push(i2);
            out.insert(i1, list.to_vec());
        } else {
            out.insert(i1, [i2].to_vec());
        }
    }

    out
}

fn check(check_data: &str, condition: HashMap<i32, Vec<i32>>) -> bool {
    let data = check_data
        .split(",")
        .filter_map(|n| n.trim().parse::<i32>().ok())
        .collect::<Vec<i32>>();
    let len = data.len();
    for i in 0..len {
        if let Some(k) = condition.get(&data[i]) {
            for c in i..len {
                if k.contains(&data[c]) {
                    return false;
                }
            }
        }
    }
    true
}
