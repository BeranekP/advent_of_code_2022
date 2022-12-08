use day_8::{calculate_score, find_visible_trees, parse_data};

const FILE: &str = "input.txt";

fn main() {
    let data = parse_data(FILE);
    let forest = find_visible_trees(data);
    let scores = calculate_score(&forest);
    println!("Part 1:{:#?}", &forest.visible.len());
    println!("Part 2: {:#?}", scores.unwrap());
}
