use day_4::{find_partial_overlaps, find_total_overlaps, parse_data};

fn main() {
    let file = "input.txt";
    let data = parse_data(file);
    let total_overlaps = find_total_overlaps(&data);
    let partial_overlaps = find_partial_overlaps(&data);
    println!("Part 1: {:#?}", total_overlaps);
    println!("Part 2: {:#?}", partial_overlaps);
}
