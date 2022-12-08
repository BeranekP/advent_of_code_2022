use std::fs::File;
use std::io::{BufRead, BufReader};

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}
#[derive(PartialEq, Debug)]
pub struct Tree {
    size: u64,
    x: usize,
    y: usize,
}

pub struct Forest {
    pub trees: Vec<Vec<u64>>,
    pub trees_tr: Vec<Vec<u64>>, // transposed trees for column checking
    pub visible: Vec<Tree>,
}

pub fn parse_data(file: &str) -> Vec<Vec<u64>> {
    let mut result = Vec::new();

    let file = File::open(file).expect("Error in reading file");

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line_by_chars = line
            .expect("Error reading line")
            .chars()
            .map(|ch| ch.to_string().parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        result.push(line_by_chars);
    }
    result
}

pub fn find_visible_trees(trees: Vec<Vec<u64>>) -> Forest {
    let mut forest = Forest {
        trees: trees.clone(),
        visible: vec![],
        trees_tr: transpose(trees.clone()),
    };

    //check rows
    for (row, row_value) in forest.trees.iter().enumerate() {
        for (column, column_value) in row_value.iter().enumerate() {
            let tree = Tree {
                size: *column_value,
                x: row,
                y: column,
            };
            if row == 0 || row == forest.trees.len() - 1 {
                if !forest.visible.contains(&tree) {
                    forest.visible.push(tree);
                }
            } else if column == 0 || column == row_value.len() - 1 {
                if !forest.visible.contains(&tree) {
                    forest.visible.push(tree);
                }
            } else {
                if forest.trees[row][..column].iter().all(|x| x < column_value)
                    || forest.trees[row][column + 1..]
                        .iter()
                        .all(|x| x < column_value)
                {
                    if !forest.visible.contains(&tree) {
                        forest.visible.push(tree);
                    }
                }
            }
        }
    }
    //check columns
    for (column, column_value) in forest.trees_tr.iter().enumerate() {
        for (row, row_value) in column_value.iter().enumerate() {
            let tree = Tree {
                size: *row_value,
                x: row,
                y: column,
            };
            if forest.trees_tr[column][..row].iter().all(|x| x < row_value)
                || forest.trees_tr[column][row + 1..]
                    .iter()
                    .all(|x| x < row_value)
            {
                if !forest.visible.contains(&tree) {
                    forest.visible.push(tree);
                }
            }
        }
    }

    forest
}

pub fn calculate_score(forest: &Forest) -> Option<u64> {
    let mut scores = vec![];
    for tree in &forest.visible {
        let row = tree.x;
        let column = tree.y;

        let len_l = forest.trees[row][..column].len() as u64;
        let len_r = forest.trees[row][column + 1..].len() as u64;
        let len_t = forest.trees_tr[column][..row].len() as u64;
        let len_b = forest.trees_tr[column][row + 1..].len() as u64;

        let mut row_score_left = forest.trees[row][..column]
            .iter()
            .rev()
            .take_while(|x| x < &&tree.size)
            .map(|_| 1)
            .sum::<u64>();
        if row_score_left < len_l {
            row_score_left += 1;
        }

        let mut row_score_right = forest.trees[row][column + 1..]
            .iter()
            .take_while(|x| x < &&tree.size)
            .map(|_| 1)
            .sum::<u64>();
        if row_score_right < len_r {
            row_score_right += 1;
        }

        //column score
        let mut column_score_top = forest.trees_tr[column][..row]
            .iter()
            .rev()
            .take_while(|x| x < &&tree.size)
            .map(|_| 1)
            .sum::<u64>();
        if column_score_top < len_t {
            column_score_top += 1;
        }
        let mut column_score_bot = forest.trees_tr[column][row + 1..]
            .iter()
            .take_while(|x| x < &&tree.size)
            .map(|_| 1)
            .sum::<u64>();
        if column_score_bot < len_b {
            column_score_bot += 1;
        }

        scores.push(row_score_left * row_score_right * column_score_top * column_score_bot);
    }
    return scores.iter().max().copied();
}
