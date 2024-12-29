mod input;
fn main() {
    let input = input::data().unwrap();
    let data = parse(input);
    let start_po = get_start(&data);
    println!("{:?}", start_po);

    let position = Position {
        row: start_po[0].0,
        col: start_po[0].1,
    };
    let mut map = Map {
        hill: data,
        position,
        start_po,
    };
    let mut result = 0;
    for i in 0..map.start_po.len() {
        let row = map.start_po[i].0;
        let col = map.start_po[i].1;
        map.position.row = row;
        map.position.col = col;
        let k = map.no_hikes();
        println!("{}", k);
        result += k;
    }
    println!("{}", result);
}
struct Map {
    hill: Vec<Vec<i32>>,
    position: Position,
    start_po: Vec<(i32, i32)>,
}
struct Position {
    row: i32,
    col: i32,
}
impl Map {
    fn no_hikes(&self) -> i32 {
        let mut noofhikes = 0;
        let row = self.position.row;
        let col = self.position.col;
        //let mut hill = self.hill.clone();
        let mut temp_vec: Vec<(i32, i32)> = vec![];
        //let mut seen_vec: Vec<(i32, i32)> = vec![];
        temp_vec.push((row, col));
        while temp_vec.len() > 0 {
            let (row, col) = temp_vec.pop().unwrap();
            let new_row = row;
            let new_col = col + 1;
            if self.is_valid(new_row, new_col) {
                if self.hill[row as usize][col as usize] + 1
                    == self.hill[new_row as usize][new_col as usize]
                {
                    if self.hill[new_row as usize][new_col as usize] == 9 {
                            noofhikes += 1;
                    } else {
                        temp_vec.push((new_row, new_col));
                    }
                }
            }
            let new_row = row + 1;
            let new_col = col;
            if self.is_valid(new_row, new_col) {
                if self.hill[row as usize][col as usize] + 1
                    == self.hill[new_row as usize][new_col as usize]
                {
                    if self.hill[new_row as usize][new_col as usize] == 9 {
                            noofhikes += 1;
                    } else {
                        temp_vec.push((new_row, new_col));
                    }
                }
            }
            let new_row = row;
            let new_col = col - 1;
            if self.is_valid(new_row, new_col) {
                if self.hill[row as usize][col as usize] + 1
                    == self.hill[new_row as usize][new_col as usize]
                {
                    if self.hill[new_row as usize][new_col as usize] == 9 {
                            noofhikes += 1;
                        
                    } else {
                        temp_vec.push((new_row, new_col));
                    }
                }
            }
            let new_row = row - 1;
            let new_col = col;

            if self.is_valid(new_row, new_col) {
                if self.hill[row as usize][col as usize] + 1
                    == self.hill[new_row as usize][new_col as usize]
                {
                    if self.hill[new_row as usize][new_col as usize] == 9 {
                            noofhikes += 1;
                    } else {
                        temp_vec.push((new_row, new_col));
                    }
                }
            }
        }
        //println!("{:?}", hill);
        noofhikes
    }
    fn is_valid(&self, row: i32, col: i32) -> bool {
        let row_len = self.hill.len() as i32;
        let col_len = self.hill[0].len() as i32;
        row_len > row && col_len > col && 0 <= row && 0 <= col
    }
}
fn parse(data: String) -> Vec<Vec<i32>> {
    let mut matrix: Vec<Vec<i32>> = vec![];
    let k: Vec<_> = data.trim().split("\n").collect();
    for i in k {
        let n: Vec<i32> = i
            .trim()
            .chars()
            .map(|a| a.to_digit(10).unwrap() as i32)
            .collect();
        matrix.push(n);
    }
    println!("{:?}", matrix);
    matrix
}
fn get_start(data: &Vec<Vec<i32>>) -> Vec<(i32, i32)> {
    let mut out: Vec<(i32, i32)> = vec![];
    for i in 0..data.len() {
        for k in 0..data[i].len() {
            if data[i][k] == 0 {
                out.push((i as i32, k as i32));
            }
        }
    }
    out
}
