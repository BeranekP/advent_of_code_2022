use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_data(file: &str) -> Vec<(String, i32)> {
    // parse input file to vec of ("command", value)

    let mut result = Vec::new();

    let file = File::open(file).expect("Error in reading file");

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line_by_chars = line
            .expect("Error reading line")
            .trim()
            .split_once(" ")
            .map(|(s1, s2)| (s1.to_string(), s2.parse::<i32>().unwrap()))
            .unwrap_or(("noop".to_string(), 0));
        result.push(line_by_chars);
    }
    result
}

pub fn run(commands: &Vec<(String, i32)>) -> Vec<i32> {
    // initial setup
    let mut cycle: usize = 0;
    let mut register: i32 = 1;
    let mut sprite: Vec<i32> = vec![register - 1, register, register + 1];
    let mut values: Vec<i32> = vec![];
    let mut display = String::new();

    //initial draw
    draw(&register, &mut cycle, &mut sprite, &mut display);

    //process all commands
    for command in commands.iter() {
        if command.0 == "noop" {
            tick(&register, &mut cycle, &mut values);
            draw(&register, &mut cycle, &mut sprite, &mut display);
        } else if command.0 == "addx" {
            tick(&register, &mut cycle, &mut values);
            draw(&register, &mut cycle, &mut sprite, &mut display);
            tick(&register, &mut cycle, &mut values);
            register += command.1;
            draw(&register, &mut cycle, &mut sprite, &mut display);
        }

        if cycle > 240 {
            break;
        }
    }
    // print display
    println!("Part 2:");
    println!("{:▬>40}", "\n");
    println!("{}", display);
    println!("{:▬>40}", "");

    values
}

fn tick(register: &i32, cycle: &mut usize, values: &mut Vec<i32>) {
    // updates cycle and register
    *cycle += 1;
    values.push(*register);
}

fn draw(register: &i32, cycle: &usize, sprite: &mut Vec<i32>, display: &mut String) {
    // updates sprite, draws to display

    sprite[0] = *register - 1;
    sprite[1] = *register;
    sprite[2] = *register + 1;

    let line_end = [39, 79, 119, 159, 199, 239];

    // normalize cycle for display width
    let norm_cycle = match *cycle {
        0..=39 => *cycle,
        40..=79 => *cycle - 40,
        80..=119 => *cycle - 80,
        120..=159 => *cycle - 120,
        160..=199 => *cycle - 160,
        200..=239 => *cycle - 200,
        _ => *cycle - 240,
    };

    // check sprite overlap with cycle
    // draw brigth pixel
    if sprite.contains(&(norm_cycle as i32)) {
        // check for display line end
        if line_end.contains(&cycle) {
            *display += "█\n";
        } else {
            *display += "█";
        }
    // draw dark pixel
    } else {
        // check for display line end
        if line_end.contains(&cycle) {
            *display += " \n";
        } else {
            *display += " ";
        }
    }
}

pub fn get_signal_strength(values: &Vec<i32>) -> i32 {
    // calculate signal strength for part 1

    let cycles = [19, 59, 99, 139, 179, 219];
    let mut sum = 0;
    for (index, value) in values.iter().enumerate() {
        if cycles.contains(&index) {
            sum += (index as i32 + 1) * value;
        }
    }
    sum
}
