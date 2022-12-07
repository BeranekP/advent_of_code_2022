use day_7::init_tree;

const TOTAL_SPACE: u64 = 70000000;
const MIN_FREE_SPACE: u64 = 30000000;
const FILE_SIZE: u64 = 100000;

fn main() {
    let file = "input.txt";
    let tree = init_tree(file);
    let mut acc = vec![];
    tree.borrow().get_total(FILE_SIZE, &mut acc);
    println!("Part 1: {}", acc.iter().sum::<u64>());

    let mut acc = vec![];
    tree.borrow().get_min_delete(0, &mut acc);
    let used_space = acc.iter().max();
    let free_space = TOTAL_SPACE - used_space.unwrap();

    let mut acc = vec![];
    tree.borrow()
        .get_min_delete(MIN_FREE_SPACE - free_space, &mut acc);

    println!("Part 2: {}", acc.iter().min().unwrap());
}
