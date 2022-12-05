use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_data(file: &str) -> Vec<Vec<(u64, u64)>> {
    let mut result = Vec::new();
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();

    let file = File::open(file).expect("Error in reading file");

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let mut line_by_ranges = Vec::new();

        for cap in re.captures_iter(&line.expect("error reading line")) {
            line_by_ranges.push((
                cap[1].parse::<u64>().unwrap(),
                cap[2].parse::<u64>().unwrap(),
            ));
            line_by_ranges.push((
                cap[3].parse::<u64>().unwrap(),
                cap[4].parse::<u64>().unwrap(),
            ));
        }
        result.push(line_by_ranges);
    }
    result
}

pub fn find_total_overlaps(ranges: &Vec<Vec<(u64, u64)>>) -> u64 {
    let mut total: u64 = 0;
    for range in ranges {
        let (r1, r2) = &range[0];
        let (r3, r4) = &range[1];
        if r1 >= r3 && r2 <= r4 {
            total += 1;
            //println!("({r3}-{r4}) contains ({r1}-{r2}): {total}");
        } else if r1 <= r3 && r2 >= r4 {
            total += 1;
            //println!("({r1}-{r2}) contains ({r3}-{r4}): {total}");
        } else {
            total += 0;
        }
    }
    total
}

pub fn find_partial_overlaps(ranges: &Vec<Vec<(u64, u64)>>) -> u64 {
    let mut total: u64 = 0;
    for range in ranges {
        let (r1, r2) = &range[0];
        let (r3, r4) = &range[1];
        if r2 >= r3 && r1 <= r3 || r1 >= r3 && r2 <= r4 {
            total += 1;
        } else if r1 <= r4 && r2 >= r4 || r1 <= r3 && r2 >= r4 {
            total += 1;
        } else {
            total += 0;
        }
    }
    total
}
