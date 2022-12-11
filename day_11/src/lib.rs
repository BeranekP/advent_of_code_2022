//use regex::Regex;
use std::collections::HashMap;
use std::fs;
//use std::fs::File;
//use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Monkey {
    pub id: usize,
    pub items: Vec<u128>,
    pub operation: (String, String),
    pub test: u128,
    pub true_case: usize,
    pub false_case: usize,
    pub inspected: u128,
}
impl Monkey {
    pub fn new(
        id: usize,
        items: Vec<u128>,
        operation: (String, String),
        test: u128,
        true_case: usize,
        false_case: usize,
    ) -> Self {
        Self {
            id,
            items,
            operation,
            test,
            true_case,
            false_case,
            inspected: 0,
        }
    }
}

pub fn parse_data(file: &str) -> HashMap<usize, Monkey> {
    // parse input file to vec of {id:Monkey}

    let string = fs::read_to_string(file).expect("Error reading to string");
    let mut monkeys: HashMap<usize, Monkey> = HashMap::new();

    // monkeys raw data
    let monkeys_raw = string
        .trim()
        .split("\n\n")
        .map(|monkey| monkey.to_string())
        .collect::<Vec<String>>();

    // process raw data
    for (i, monkey) in monkeys_raw.iter().enumerate() {
        let m = monkey
            .split("\n")
            .map(|m| m.trim().to_string())
            .collect::<Vec<String>>();

        let items = m[1]
            .split_once(": ")
            .map(|(_, numbers)| numbers.split(", ").map(|n| n.parse::<u128>().unwrap()))
            .unwrap()
            .collect::<Vec<u128>>();

        let operation = m[2]
            .split_once("= old ")
            .unwrap()
            .1
            .split_once(" ")
            .unwrap();
        let operator = operation.0;
        let operand = operation.1;
        let test = m[3]
            .matches(char::is_numeric)
            .collect::<String>()
            .parse::<u128>()
            .unwrap();
        let true_case = m[4]
            .matches(char::is_numeric)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        let false_case = m[5]
            .matches(char::is_numeric)
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        // insert new monkey
        monkeys.entry(i).or_insert(Monkey::new(
            i,
            items.clone(),
            (operator.to_string(), operand.to_string()),
            test,
            true_case,
            false_case,
        ));
    }

    monkeys
}
fn high_worry_item(item: &mut u128, (operation, operand): &(String, String), monkey_mod: u128) {
    // value of item if worry not reduced by 3
    match (operation.as_str(), operand.as_str()) {
        ("+", operand) => {
            if operand == "old" {
                *item = ((*item % monkey_mod) + (*item % monkey_mod)) % monkey_mod;
            } else {
                *item = ((*item % monkey_mod) + (operand.parse::<u128>().unwrap() % monkey_mod))
                    % monkey_mod;
            }
        }
        ("*", operand) => {
            if operand == "old" {
                *item = ((*item % monkey_mod) * (*item % monkey_mod)) % monkey_mod;
            } else {
                *item = ((*item % monkey_mod) * (operand.parse::<u128>().unwrap() % monkey_mod))
                    % monkey_mod;
            }
        }
        _ => {}
    }
}
fn low_worry_item(item: &mut u128, (operation, operand): &(String, String)) {
    match (operation.as_str(), operand.as_str()) {
        ("+", operand) => {
            if operand == "old" {
                *item = *item + *item;
            } else {
                *item = *item + operand.parse::<u128>().unwrap();
            }
        }
        ("*", operand) => {
            if operand == "old" {
                *item = *item * *item;
            } else {
                *item = *item * operand.parse::<u128>().unwrap();
            }
        }
        _ => {}
    }
}

pub fn keep_away(monkeys: &mut HashMap<usize, Monkey>, rounds: u64, worry: bool) {
    let mut round = 0;

    // common product
    let monkey_mod: u128 = if worry {
        monkeys.iter().map(|(_, v)| v.test).product()
    } else {
        1
    };

    //main loop
    loop {
        if round > rounds - 1 {
            break;
        }
        let length = monkeys.keys().len();
        for i in 0..length {
            //run while monkey has items
            while !monkeys[&i].items.is_empty() {
                let mut item = monkeys.get_mut(&i).unwrap().items.remove(0);
                if !worry {
                    low_worry_item(&mut item, &monkeys[&i].operation);
                    item = item / 3;
                } else {
                    high_worry_item(&mut item, &monkeys[&i].operation, monkey_mod);
                }

                // throw if true
                if item % monkeys[&i].test == 0 {
                    let index = monkeys[&i].true_case;
                    let mut items = monkeys[&index].items.clone();
                    items.push(item);
                    monkeys.get_mut(&index).unwrap().items = items;

                // throw if false
                } else {
                    let index = monkeys[&i].false_case;
                    let mut items = monkeys[&index].items.clone();
                    items.push(item);
                    monkeys.get_mut(&index).unwrap().items = items;
                }

                // increase number of inspected items for current monkey
                monkeys.get_mut(&i).unwrap().inspected += 1;
            }
        }
        round += 1
    }
}

pub fn monkey_business(monkeys: &HashMap<usize, Monkey>) -> u128 {
    // calculates product of two 'best' monkeys

    let mut values = vec![];
    for (_, monkey) in monkeys {
        values.push(monkey.inspected);
    }

    values.sort();
    values.reverse();

    values.iter().take(2).product()
}
