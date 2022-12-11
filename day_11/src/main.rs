use day_11::{keep_away, monkey_business, parse_data};

const FILE: &str = "input.txt";
const WORRY: bool = false;
const ROUNDS: u64 = 20;

fn main() {
    let mut monkeys = parse_data(FILE);

    keep_away(&mut monkeys, ROUNDS, WORRY);
    let score = monkey_business(&monkeys);
    println!("{}", score);
}
