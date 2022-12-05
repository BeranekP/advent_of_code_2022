use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Stacks {
    pub data: HashMap<usize, Vec<char>>,
    pub end_file: usize,
}

impl Stacks {
    pub fn build_from_file(file: &str) -> Stacks {
        let (stacks, end) = get_stacks(file);
        Stacks {
            data: stacks,
            end_file: end,
        }
    }
    pub fn get_result(&self) -> String {
        let mut result = vec![' '; self.data.keys().len()];
        for (key, value) in &self.data {
            result[key - 1] = value.last().copied().unwrap();
        }
        result.iter().collect()
    }
}

pub struct CrateMover {
    model: u64,
    pub moves: Vec<Vec<usize>>,
}

impl CrateMover {
    pub fn build_from_file(file: &str, start: usize, model: u64) -> Self {
        CrateMover {
            model: model,
            moves: get_moves(file, start),
        }
    }

    pub fn run(&self, stacks: &mut Stacks) {
        match &self.model {
            9000 => make_single_moves(&self.moves, &mut stacks.data),
            9001 => make_multi_moves(&self.moves, &mut stacks.data),
            _ => {}
        }
    }
}

// Util functions

// parse stacks from input file into hashmap and also return line number, where stack data ends

fn get_stacks(file: &str) -> (HashMap<usize, Vec<char>>, usize) {
    let mut stacks = HashMap::new();
    let mut end = 0;
    let file = File::open(file).expect("Error in reading file");

    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        if !line.as_ref().expect("Error reading line").is_empty() {
            let line_by_chars = line
                .expect("Error reading line")
                .chars()
                .collect::<Vec<char>>();
            let n_stacks = (&line_by_chars.len() + 2) / 4;
            for i in (1..n_stacks * 4).step_by(4) {
                if line_by_chars[i] != ' ' && !line_by_chars[i].is_numeric() {
                    stacks
                        .entry(i / 4 + 1)
                        .or_insert_with(Vec::new)
                        .insert(0, line_by_chars[i]);
                }
            }
        } else {
            end = i;
            break;
        }
    }
    (stacks, end)
}

// get moves from input file, starting where stacks ended
fn get_moves(file: &str, start: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();

    let file = File::open(file).expect("Error in reading file");
    let reader = BufReader::new(file);

    let re = Regex::new(r"(\d+)(?:\D+)(\d+)(?:\D+)(\d+)").unwrap();

    for line in reader.lines().skip(start + 1) {
        let mut line_by_ranges = Vec::new();

        for cap in re.captures_iter(&line.expect("error reading line")) {
            line_by_ranges.append(
                vec![
                    cap[1].parse::<usize>().unwrap(),
                    cap[2].parse::<usize>().unwrap(),
                    cap[3].parse::<usize>().unwrap(),
                ]
                .as_mut(),
            );
        }
        result.push(line_by_ranges);
    }
    result
}

fn make_single_moves(moves: &Vec<Vec<usize>>, stacks: &mut HashMap<usize, Vec<char>>) {
    for m in moves {
        let [number, from, to] = [m[0], m[1], m[2]];
        for _ in 0..number {
            let popped = stacks.get_mut(&from).unwrap().pop().expect("Error popping");
            stacks.get_mut(&to).unwrap().push(popped);
        }
    }
}
fn make_multi_moves(moves: &Vec<Vec<usize>>, stacks: &mut HashMap<usize, Vec<char>>) {
    for m in moves {
        let [number, from, to] = [m[0], m[1], m[2]];
        let items = stacks[&from].len();
        let mut popped = Vec::from(&stacks.get_mut(&from).unwrap().clone()[items - number..]);
        stacks.get_mut(&from).unwrap().truncate(items - number);
        stacks.get_mut(&to).unwrap().append(&mut popped);
    }
}
