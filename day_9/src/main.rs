use day_9::{find_path, parse_data, print_field};

const FILE: &str = "input.txt";

fn main() {
    let data = parse_data(FILE);
    let visited_part1 = find_path(&data, 2);
    let visited_part2 = find_path(&data, 10);
    //println!("{:?}", field);
    println!("Part 1: {:?}", visited_part1.len());
    println!("Part 2: {:?}", visited_part2.len());
    //print_field(&field);
}

