use day_1::Elves;

fn main() {
    let input = "input.txt";
    let elves = Elves::new(input);

    println!("Top elf: {:#?}", elves.get()[0]);
    println!("Sum top 3: {:#?}", elves.sum());
}
