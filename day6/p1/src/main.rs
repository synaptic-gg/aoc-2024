mod input;

fn main() {
    let input = input::data().unwrap();
    let room_data = parse_input(input);
    let position = Position::new(&room_data).unwrap();
    let mut room = Room {
        room: room_data,
        position,
        run_status: true,
    };
    //println!("{:?}", room.room);
    while room.run_status {
        room.move_next();
        //println!("{} , {} , {}",room.position.row, room.position.col, room.position.direction);
    }
    let mut result = 0;

    for lines in room.room {
        for cha in lines {
            if cha == 'X' {
                result += 1;
            };
        }
    }

    println!("{}", result)
}

struct Room {
    room: Vec<Vec<char>>,
    position: Position,
    run_status: bool,
}

struct Position {
    row: i32,
    col: i32,
    direction: char,
}

impl Room {
    fn move_next(&mut self) {
        let mut next = '/';
        match self.position.direction {
            '>' => next = self.room[self.position.row as usize][(self.position.col + 1) as usize],
            '<' => next = self.room[self.position.row as usize][(self.position.col - 1) as usize],
            'v' => next = self.room[(self.position.row + 1) as usize][self.position.col as usize],
            '^' => next = self.room[(self.position.row - 1) as usize][self.position.col as usize],
            _ => println!("Not a direction"),
        };

        if next == '/' {
            self.room[self.position.row as usize][self.position.col as usize] = 'X';
            self.run_status = false;
        } else if next == '#' {
            self.position.turn_right();
        } else {
            self.room[self.position.row as usize][self.position.col as usize] = 'X';
            self.position.move_f();
        }
    }
}

impl Position {
    fn new(data: &Vec<Vec<char>>) -> Option<Position> {
        let possible_directions = ['^', '>', '<', 'v'];
        for line in data.iter().enumerate() {
            for cha in line.1.iter().enumerate() {
                if possible_directions.contains(cha.1) {
                    return Some(Position {
                        row: line.0 as i32,
                        col: cha.0 as i32,
                        direction: *cha.1,
                    });
                }
            }
        }
        None
    }
    fn move_f(&mut self) {
        match self.direction {
            '>' => self.col += 1,
            '<' => self.col -= 1,
            'v' => self.row += 1,
            '^' => self.row -= 1,
            _ => println!("Not a direction"),
        };
    }
    fn turn_right(&mut self) {
        match self.direction {
            '>' => self.direction = 'v',
            '<' => self.direction = '^',
            'v' => self.direction = '<',
            '^' => self.direction = '>',
            _ => println!("Not a direction"),
        };
    }
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
