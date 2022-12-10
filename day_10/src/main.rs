use day_10::{get_signal_strength, parse_data, run};

const FILE: &str = "input.txt";

fn main() {
    let data = parse_data(FILE);
    let values = run(&data);

    let strength = get_signal_strength(&values);
    println!("Part 1: {:?}", strength);
}
