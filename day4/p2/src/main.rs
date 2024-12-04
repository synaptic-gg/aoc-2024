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
            if check(r, c, 'M', &matrix)
                && check(r, c + 2, 'M', &matrix)
                && check(r + 1, c + 1, 'A', &matrix)
                && check(r + 2, c, 'S', &matrix)
                && check(r + 2, c + 2, 'S', &matrix)
            {
                result += 1
            }
            if check(r, c, 'M', &matrix)
                && check(r, c + 2, 'S', &matrix)
                && check(r + 1, c + 1, 'A', &matrix)
                && check(r + 2, c, 'M', &matrix)
                && check(r + 2, c + 2, 'S', &matrix)
            {
                result += 1
            }
            if check(r, c, 'S', &matrix)
                && check(r, c + 2, 'S', &matrix)
                && check(r + 1, c + 1, 'A', &matrix)
                && check(r + 2, c, 'M', &matrix)
                && check(r + 2, c + 2, 'M', &matrix)
            {
                result += 1
            }
            if check(r, c, 'S', &matrix)
                && check(r, c + 2, 'M', &matrix)
                && check(r + 1, c + 1, 'A', &matrix)
                && check(r + 2, c, 'S', &matrix)
                && check(r + 2, c + 2, 'M', &matrix)
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
