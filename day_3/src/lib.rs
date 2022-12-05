use sets::Set;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_data(file: &str) -> Vec<Vec<char>> {
    let mut result = Vec::new();

    let file = File::open(file).expect("Error in reading file");

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line_by_chars = line
            .expect("Error reading line")
            .chars()
            .collect::<Vec<char>>();
        result.push(line_by_chars);
    }
    result
}

pub fn find_duplicates_rucksacks(rucksacks: &Vec<Vec<char>>) -> u64 {
    let mut sum: u64 = 0;
    for rucksack in rucksacks {
        let part1 = Set::new_unordered(&rucksack[..rucksack.len() / 2]);
        let part2 = Set::new_unordered(&rucksack[rucksack.len() / 2..]);
        let intersection = part1.intersection(&part2);
        let duplicate = intersection.data[0];
        sum += calculate_score(duplicate) as u64;
    }
    sum
}

pub fn find_badge_rucksacks(rucksacks: &Vec<Vec<char>>) -> u64 {
    let mut sum: u64 = 0;
    for rucksack in rucksacks.chunks(3) {
        let r1 = Set::new_unordered(&rucksack[0]);
        let r2 = Set::new_unordered(&rucksack[1]);
        let r3 = Set::new_unordered(&rucksack[2]);
        let r1_r2 = r1.intersection(&r2);
        let badge = r1_r2.intersection(&r3);
        sum += calculate_score(badge.data[0]) as u64;
    }
    sum
}

pub fn calculate_score(c: char) -> u8 {
    if c.is_lowercase() {
        c as u8 - 96
    } else if c.is_uppercase() {
        c as u8 - 64 + 26
    } else {
        0
    }
}
