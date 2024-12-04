mod input;
fn main() {
    //let data = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";
    let data = input::data().unwrap();
    let level = data.split("\n").collect::<Vec<_>>();
    let mut matrix: Vec<Vec<char>> = vec![];
    for i in 0..level.len() {
        matrix.push(level[i].chars().collect::<Vec<char>>());
    }
    let mut result = 0;
    for c in 0..(matrix.len() as i32) {
        for r in 0..(matrix[0].len() as i32) {
            if check(r + 0, c + 0, 'X', &matrix)
                && check(r + 0, c + 1, 'M', &matrix)
                && check(r + 0, c + 2, 'A', &matrix)
                && check(r + 0, c + 3, 'S', &matrix)
            {
                result += 1
            }
            if check(r + 0, c + 0, 'X', &matrix)
                && check(r + 0, c - 1, 'M', &matrix)
                && check(r + 0, c - 2, 'A', &matrix)
                && check(r + 0, c - 3, 'S', &matrix)
            {
                result += 1
            }
            if check(r + 0, c + 0, 'X', &matrix)
                && check(r + 1, c + 0, 'M', &matrix)
                && check(r + 2, c + 0, 'A', &matrix)
                && check(r + 3, c + 0, 'S', &matrix)
            {
                result += 1
            }
            if check(r + 0, c + 0, 'X', &matrix)
                && check(r - 1, c, 'M', &matrix)
                && check(r - 2, c, 'A', &matrix)
                && check(r - 3, c, 'S', &matrix)
            {
                result += 1
            }
            if check(r + 0, c + 0, 'X', &matrix)
                && check(r + 1, c + 1, 'M', &matrix)
                && check(r + 2, c + 2, 'A', &matrix)
                && check(r + 3, c + 3, 'S', &matrix)
            {
                result += 1
            }
            if check(r + 0, c + 0, 'X', &matrix)
                && check(r - 1, c - 1, 'M', &matrix)
                && check(r - 2, c - 2, 'A', &matrix)
                && check(r - 3, c - 3, 'S', &matrix)
            {
                result += 1
            }
            if check(r + 0, c + 0, 'X', &matrix)
                && check(r - 1, c + 1, 'M', &matrix)
                && check(r - 2, c + 2, 'A', &matrix)
                && check(r - 3, c + 3, 'S', &matrix)
            {
                result += 1
            }
            if check(r + 0, c + 0, 'X', &matrix)
                && check(r + 1, c - 1, 'M', &matrix)
                && check(r + 2, c - 2, 'A', &matrix)
                && check(r + 3, c - 3, 'S', &matrix)
            {
                result += 1
            }
        }
    }

    println!("{}", result);
}
fn check(row: i32, column: i32, val: char, data: &Vec<Vec<char>>) -> bool {
    if let Some(k) = &data.get(row as usize) {
        if let Some(p) = k.get(column as usize) {
            return val == *p;
        }
    }
    false
}
