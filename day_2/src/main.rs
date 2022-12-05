use day_2::{parse_data, process_strategy};

fn main() {
    let file = "./input.txt";
    let strategy = parse_data(file);
    let result1 = process_strategy(&strategy, 1);
    let result2 = process_strategy(&strategy, 2);

    println!("Strategy 1: {:#?}", result1);
    println!("Strategy 2: {:#?}", result2);
}
