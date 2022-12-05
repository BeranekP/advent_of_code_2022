use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Elves {
    first: u64,
    second: u64,
    third: u64,
}
impl Elves {
    pub fn new(file: &str) -> Elves {
        let mut elves = Elves {
            first: 0,
            second: 0,
            third: 0,
        };
        elves.build(file);
        elves
    }

    pub fn get(&self) -> Vec<u64> {
        vec![self.first, self.second, self.third]
    }
    pub fn sum(&self) -> u64 {
        self.first + self.second + self.third
    }

    fn build(&mut self, file: &str) {
        let mut result = Vec::new();

        let file = File::open(file).expect("Error in reading file");
        let reader = BufReader::new(file);
        for line in reader.lines() {
            result.push(
                line.expect("Error reading line")
                    .parse::<u64>()
                    .unwrap_or_else(|_| 0),
            )
        }
        //result;
        let _ = &self.get_max_calories(result);
    }

    fn get_max_calories(&mut self, list: Vec<u64>) {
        let mut sum = 0;
        let mut max_sum = 0;
        for item in list {
            if item > 0 {
                sum += item;
            } else {
                if sum > max_sum {
                    max_sum = sum;
                }
                let _ = &self.get_top_three(sum);
                sum = 0;
            };
        }
    }
    fn get_top_three(&mut self, max_sum: u64) {
        if max_sum > self.first {
            self.first = max_sum;
        } else if self.second < max_sum && max_sum <= self.first {
            self.second = max_sum;
        } else if self.third < max_sum && max_sum <= self.second {
            self.third = max_sum;
        }
    }
}
