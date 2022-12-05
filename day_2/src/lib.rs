use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_data(file: &str) -> Vec<String> {
    let mut result = Vec::new();

    let file = File::open(file).expect("Error in reading file");

    let reader = BufReader::new(file);
    for line in reader.lines() {
        result.push(line.expect("Error reading line"));
    }
    result
}

pub fn process_strategy(strategy: &Vec<String>, key: u64) -> u64 {
    let mut score: u64 = 0;
    for line in strategy {
        let round = line.trim();

        if key == 1 {
            score += calculate_score(round)
        } else {
            score += calculate_score2(round)
        }
    }
    score
}
fn calculate_score(round: &str) -> u64 {
    let score = match round {
        "A X" => 4, // Rock - Rock, 1(rock) + 3(draw)
        "A Y" => 8, // Rock - Paper, 2(paper) + 6(win)
        "A Z" => 3, // Rock - Scissors, 3(scissors) + 0(lose)
        "B X" => 1, // Paper - Rock, 1(rock) + 0(lose)
        "B Y" => 5, // Paper - Paper, 2(paper) + 3(draw)
        "B Z" => 9, // Paper - Scissors, 3(scissors) + 6(win)
        "C X" => 7, // Scissors - Rock, 1(rock) + 6(win)
        "C Y" => 2, // Scissors - Paper, 2(paper) + 0(lose)
        "C Z" => 6, // Scissors - Scissors, 3(scissors) + 3(draw)
        _ => 0,
    };
    score
}

fn calculate_score2(round: &str) -> u64 {
    // X - lose
    // Y - draw
    // Z - win

    let score = match round {
        "A X" => 3, // Rock - Scissors, 3(scissors) + 0(lose)
        "A Y" => 4, // Rock - Rock, 1(rock) + 3(draw)
        "A Z" => 8, // Rock - Paper, 2(paper) + 6(win)
        "B X" => 1, // Paper - Rock, 1(rock) + 0(lose)
        "B Y" => 5, // Paper - Paper, 2(paper) + 3(draw)
        "B Z" => 9, // Paper - Scissors, 3(scissors) + 6(win)
        "C X" => 2, // Scissors - Paper, 2(paper) + 0(lose)
        "C Y" => 6, // Scissors - Scissors, 3(scissors) + 3(draw)
        "C Z" => 7, // Scissors - Rock, 1(rock) + 6(win)
        _ => 0,
    };
    score
}
