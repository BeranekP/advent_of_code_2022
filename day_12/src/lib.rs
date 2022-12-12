use std::fs;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub i32, pub i32);

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd)]
pub struct Successor {
    pub pos: Pos,
    pub cost: u32,
}

pub struct Board {
    pub width: u16,
    pub height: u16,
    pub data: Vec<Vec<Option<u8>>>,
    pub start: Vec<Pos>,
    pub end: Pos,
}

impl Board {
    pub fn new(board_rows: &Vec<String>) -> Board {
        let width = board_rows[0].len() as u16;
        let height = board_rows.len() as u16;
        let mut data = Vec::new();
        let mut start = Vec::new();
        let mut end = Pos(0, 0);
        for (i, board_row) in board_rows.iter().enumerate() {
            let mut row: Vec<Option<u8>> = Vec::new();
            for (j, c) in board_row.chars().enumerate() {
                match c {
                    // add postions based on their ASCII value
                    // mark start position and move it to front so it is the first result
                    'S' => {
                        row.push(Some('a' as u8 - 96));
                        start.insert(0, Pos(j as i32, i as i32));
                    }
                    //mark end position
                    'E' => {
                        row.push(Some('z' as u8 - 96));
                        end = Pos(j as i32, i as i32);
                    }
                    //add all a as posible starts
                    'a' => {
                        row.push(Some('a' as u8 - 96));
                        start.push(Pos(j as i32, i as i32));
                    }
                    //add rest of the board
                    'b'..='z' => row.push(Some(c as u8 - 96)),
                    _ => panic!("Not valid letter"),
                }
            }
            data.push(row);
        }
        Board {
            width,
            height,
            data,
            start,
            end,
        }
    }

    pub fn get_successors(&self, position: &Pos) -> Vec<Successor> {
        let mut successors = Vec::new();
        // get current value on the board for restriction check
        let old_value = self.data[position.1 as usize][position.0 as usize].unwrap();

        // get all possible neighbors
        for dx in -1i32..=1 {
            for dy in -1i32..=1 {
                // discard diagonal moves
                if (dx + dy).abs() != 1 {
                    continue;
                }
                let new_position = Pos(position.0 + dx, position.1 + dy);
                //check if new position is legal, i.e. on the board
                if new_position.0 < 0
                    || new_position.0 >= self.width.into()
                    || new_position.1 < 0
                    || new_position.1 >= self.height.into()
                {
                    continue;
                }
                let board_value = self.data[new_position.1 as usize][new_position.0 as usize];

                if let Some(board_value) = board_value {
                    // check if new value is larger than old value + 1
                    if board_value <= old_value + 1 {
                        successors.push(Successor {
                            pos: new_position,
                            cost: board_value as u32,
                        });
                    } else {
                        continue;
                    }
                }
            }
        }

        successors
    }
}

pub fn parse_data(file: &str) -> Vec<String> {
    // parse input file to vec of board lines

    let output: Vec<String> = fs::read_to_string(file)
        .expect("Error reading to string")
        .trim()
        .split("\n")
        .map(str::to_string)
        .collect();
    output
}
