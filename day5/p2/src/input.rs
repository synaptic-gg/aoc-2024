use std::fs::File;
use std::io::prelude::*;

pub fn data() -> std::io::Result<String> {
    //let mut file = File::open("input.txt")?;
    //let mut content = String::new();
    //file.read_to_string(&mut content)?;
    let content = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n
75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47".to_string();

    Ok(content)
}
