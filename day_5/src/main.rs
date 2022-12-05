use ::day_5::{CrateMover, Stacks};

fn main() {
    const FILE: &str = "input.txt";
    const MODEL: u64 = 9000; //9000 .. single crate mover, 9001 multi crate mover

    let mut stacks = Stacks::build_from_file(FILE);

    let crate_mover = CrateMover::build_from_file(FILE, stacks.end_file, MODEL);

    crate_mover.run(&mut stacks);

    println!("{:#?}", stacks.get_result());
}
