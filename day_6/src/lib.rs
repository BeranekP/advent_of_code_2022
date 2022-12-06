use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_data(file: &str) -> Vec<char> {
    let mut result = Vec::new();

    let file = File::open(file).expect("Error in reading file");

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line_by_chars = line
            .expect("Error reading line")
            .chars()
            .collect::<Vec<char>>();
        result = line_by_chars;
    }
    result
}

pub fn find_marker(signal: &Vec<char>, marker_len: usize) -> usize {
    let mut marker = Vec::new();
    for (i, c) in signal.iter().enumerate() {
        if marker.len() < marker_len {
            if marker.contains(c) {
                //find repeated character in marker vec
                let index = marker.iter().position(|&r| r == *c).unwrap();
                //clear chars up to repeated (included)
                marker.drain(..=index);
                marker.push(*c);
            } else {
                marker.push(*c);
            }
        } else if marker.len() == marker_len {
            return i;
        } else if marker.len() > marker_len {
            break;
        }
    }
    0
}
