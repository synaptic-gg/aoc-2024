
mod input;
use std::collections::{HashMap, HashSet};

pub fn main() {
    let in_data = input::data().unwrap();
    let mut data = in_data.split("\n\n");
    let con = data.next().unwrap();
    let check_data = data.next().unwrap();
    let hash_con = con_hash(con);
    let pages = check_data.split("\n").into_iter();
    let mut result = 0;
    for page in pages {
        let checked = check(page, &hash_con);
        if !checked {
        
            let data = sort(page,&hash_con);

            let y = data.len() / 2;
            //println!("{:?}", data);
            if data.len() != 0 {
                result += data[y]
            }
        }
    }

    println!("{}", result)
}



pub fn build_graph(rules: &[(i32, i32)], update: &[i32]) -> HashMap<i32, Vec<i32>> {
    let page_set: HashSet<i32> = update.iter().cloned().collect();
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();

    for &page in update {
        graph.entry(page).or_default();
    }

    for &(a, b) in rules {
        if page_set.contains(&a) && page_set.contains(&b) {
            graph.get_mut(&a).unwrap().push(b);
        }
    }

    graph
}

pub fn dfs(node:i32,graph: &HashMap<i32,Vec<i32>> , visted: &mut HashSet<i32>,temp : &mut HashSet<i32>,sorted : &mut  Vec<i32> )-> bool {
    if temp.contains(&node){
        return  false;
    }
    if !visted.contains(&node){
        temp.insert(node);
        for n in &graph[&node]{
            if !dfs(*n, graph, visted, temp, sorted){
                return  false;
            }
        }
        temp.remove(&node);
        visted.insert(node); 
        sorted.push(node);
    }
    true
}

pub fn con_hash(con_data: &str) -> HashMap<i32, Vec<i32>> {
    let mut out: HashMap<i32, Vec<i32>> = HashMap::new();
    let con = con_data.split("\n").into_iter();
    for i in con {
        let mut val = i
            .split("|")
            .filter_map(|n| Some(n.trim().parse::<i32>().unwrap()));
        let i2 = val.next().unwrap();
        let i1 = val.next().unwrap();
       out.entry(i1).or_default().push(i2);
    }
//println!("{:?}",out);
    out
}

pub fn sort(page: &str, condition: &HashMap<i32, Vec<i32>>) -> Vec<i32> {
    let data = page
        .split(",")
        .filter_map(|n| n.trim().parse::<i32>().ok())
        .collect::<Vec<i32>>();
    let mut rules = vec![];
    for (k, v) in condition.iter() {
        for to in v {
            rules.push((*k, *to));
        }
    }

    let graph = build_graph(&rules, &data);
    let mut visited = HashSet::new();
    let mut temp = HashSet::new();
    let mut sorted = vec![];

    for &node in &data {
        if !visited.contains(&node) {
           let _ =  !dfs(node, &graph, &mut visited, &mut temp, &mut sorted) ;
                
            
        }
    }

    sorted.reverse();
    sorted
}


pub fn check(check_data: &str, condition: &HashMap<i32, Vec<i32>>) -> bool {
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
