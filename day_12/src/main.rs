use ::day_12::{parse_data, Board};
use pathfinding::prelude::dijkstra;

const FILE: &str = "input.txt";

fn main() {
    // input data as Vec of String
    let data = parse_data(FILE);

    // build board with all possible starting positions
    let board = Board::new(&data);

    // store path length for all starting position
    let mut path_lenghts = Vec::new();

    // get start positions from board
    let starts = board.start.clone();

    // perform Dijkstra algorithm for all starting positions
    for start in starts {
        let result = dijkstra(
            &start,
            |p| {
                board
                    .get_successors(p)
                    .iter()
                    .map(|s| (s.pos, s.cost))
                    .collect::<Vec<_>>()
            },
            |p| *p == board.end,
        );
        match result {
            Some(r) => path_lenghts.push(r.0.len() - 1), // do not count end position
            None => continue,                            // Path may not be found, skip it
        };
    }

    // print results
    println!("Part 1:");
    println!("Steps to end: {:?}", path_lenghts[0]);
    println!("------------------");
    println!("Part 2:");
    println!("Steps to end: {:?}", path_lenghts.iter().min().unwrap());
}
