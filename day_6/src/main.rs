use day_6::{find_marker, parse_data};

fn main() {
    const FILE: &str = "input.txt";
    let signal = parse_data(FILE);
    let start_of_packet = find_marker(&signal, 4);
    let start_of_message = find_marker(&signal, 14);
    println!("Part 1: {:?}", start_of_packet);
    println!("Part 2: {:?}", start_of_message);
}
