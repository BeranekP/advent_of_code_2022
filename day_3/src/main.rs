use day_3::{find_badge_rucksacks, find_duplicates_rucksacks, parse_data};

fn main() {
    let file = "input.txt";

    let rucksacks = parse_data(file);
    let result1 = find_duplicates_rucksacks(&rucksacks);
    let result2 = find_badge_rucksacks(&rucksacks);

    println!("Part 1: {result1}");
    println!("Part 2: {result2}");
}
