mod input;
fn main() {
    //let mut data:HashMap<&str, Vec<i32>> = HashMap::new();
    //data.insert("list1", [7 ,6 ,4 ,2 ,1].to_vec());
    //data.insert("list2", [1 ,2 ,7 ,8 ,9].to_vec());
    //data.insert("list3", [9 ,7 ,6, 2, 1].to_vec());
    //data.insert("list4", [1 ,3 ,2 ,4 ,5].to_vec());
    //data.insert("list5", [8 ,6 ,4 ,4,1].to_vec());
    //data.insert("list2", [1 ,3 ,6 ,7 ,9].to_vec());
    let data = input::data().unwrap();
    let mut safe_no = 0;
    for (_key, value) in data {
        if issafe(value) {
            safe_no += 1
        }
    }
    println!("total no of safe lists are {}", safe_no)
}
fn issafe(s_data: Vec<i32>) -> bool {
    #[derive(PartialEq)]
    enum State {
        Inc,
        Dec,
        Non,
    }
    let mut state = State::Non;
    for i in 1..s_data.len() {
        if s_data[i - 1] < s_data[i]
            && (state == State::Non || state == State::Inc)
            && ((s_data[i] - s_data[i - 1]).abs() < 4)
        {
            state = State::Inc;
        } else if s_data[i - 1] > s_data[i]
            && (state == State::Non || state == State::Dec)
            && ((s_data[i] - s_data[i - 1]).abs() < 4)
        {
            state = State::Dec;
        } else {
            return false;
        }
    }
    true
}
