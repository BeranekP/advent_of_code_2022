use console_engine::pixel;
use console_engine::Color;
use console_engine::KeyCode;

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_data(file: &str) -> Vec<Vec<String>> {
    let mut result = Vec::new();

    let file = File::open(file).expect("Error in reading file");

    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line_by_chars = line
            .expect("Error reading line")
            .trim()
            .split(" ")
            .map(|ch| ch.to_string())
            .collect::<Vec<String>>();
        result.push(line_by_chars);
    }
    result
}

pub fn find_path(commands: &Vec<Vec<String>>, n_knots: usize) -> HashSet<(i64, i64)> {
    let mut visited = HashSet::new();
    //let mut tail: Vec<i64> = vec![0, 0];
    //let mut head: Vec<i64> = vec![0, 0];
    let mut knots = vec![vec![0, 0]; n_knots];
    for command in commands {
        let (direction, steps) = if let [direction, steps, ..] = &command[..] {
            (direction, steps.parse::<usize>().unwrap())
        } else {
            todo!()
        };
        match direction.as_str() {
            "R" => {
                for _ in 1..=steps {
                    knots[0][0] += 1;
                    let mut head = knots[0].clone();
                    for i in 0..knots.len()-1 {
                        follow_head(&head, &mut knots[i + 1]);

                        if i < knots.len() - 1 {
                            head = knots[i+1].clone();
                        }
                    }
                    tail_to_visited(&mut visited, &knots[n_knots - 1]);
                    //println!("R> {:?}", &knots);
                }
            }
            "L" => {
                for _ in 1..=steps {
                    knots[0][0] -= 1;
                    let mut head = knots[0].clone();
                    for i in 0..knots.len() - 1 {
                        follow_head(&head, &mut knots[i + 1]);

                        if i < knots.len() - 1 {
                            head = knots[i+1].clone();
                        }
                        //println!("L> H:{:?}, T:{:?}", &head, &tail);
                    }
                    tail_to_visited(&mut visited, &knots[n_knots - 1]);
                    //println!("L> {:?}", &knots);
                }
            }
            "U" => {
                for _ in 1..=steps {
                    knots[0][1] += 1;
                    let mut head = knots[0].clone();
                    for i in 0..knots.len() - 1 {
                        follow_head(&head, &mut knots[i + 1]);
                        if i < knots.len() - 1 {
                            head = knots[i+1].clone();
                        } 
                    }
                    tail_to_visited(&mut visited, &knots[n_knots - 1]);
                    //println!("U> {:?}", &knots);
                }
            }

            "D" => {
                for _ in 1..=steps {
                    
                    knots[0][1] -= 1;
                    let mut head = knots[0].clone();
                    for i in 0..knots.len() - 1 {
                        follow_head(&head, &mut knots[i + 1]);
                        if i < knots.len() - 1 {
                            head = knots[i+1].clone();
                        } 
                    }
                    tail_to_visited(&mut visited, &knots[n_knots - 1]);
                    //println!("D> {:?}", &knots);
                }
            }
            _ => {}
        }
    }
    visited
}

fn follow_head(head: &Vec<i64>, tail: &mut Vec<i64>) {
    let head_x = head[0];
    let head_y = head[1];

    let tail_x = tail[0];
    let tail_y = tail[1];

    // directly left
    if head_x - tail_x < -1 && tail_y == head_y {
        tail[0] -= 1;
    }
    //diagonal left - up
    else if head_x - tail_x < -1 && tail_y < head_y || head_y - tail_y > 1 && tail_x > head_x {
        tail[0] -= 1;
        tail[1] += 1;
    }
    //diagonal left - down
    else if head_x - tail_x < -1 && tail_y > head_y || head_y - tail_y < -1 && tail_x > head_x {
        tail[0] -= 1;
        tail[1] -= 1;
    }
    //directly right
    else if head_x - tail_x > 1 && tail_y == head_y {
        tail[0] += 1;
    }
    //diagonal right - up
    else if head_x - tail_x > 1 && tail_y < head_y || head_y - tail_y > 1 && tail_x < head_x {
        tail[0] += 1;
        tail[1] += 1;
    }
    //diagonal right - down
    else if head_x - tail_x > 1 && tail_y > head_y || head_y - tail_y < -1 && tail_x < head_x {
        tail[0] += 1;
        tail[1] -= 1;
    }
    //directly up
    else if head_y - tail_y > 1 && tail_x == head_x {
        tail[1] += 1;
    }
    //directly down
    else if head_y - tail_y < -1 && tail_x == head_x {
        tail[1] -= 1;
    }
}

fn tail_to_visited(visited: &mut HashSet<(i64, i64)>, tail: &Vec<i64>) {
    let visited_tail = (tail[0], tail[1]);
    visited.insert(visited_tail);
    
}

pub fn print_field(field:&HashSet<(i64, i64)>){
let max_x = field.into_iter().map(|(v,_)|*v ).fold(0, std::cmp::max);
let max_y = field.into_iter().map(|(_,v)|*v ).fold(0, std::cmp::max);
let min_x = field.into_iter().map(|(v,_)|*v ).fold(0, std::cmp::min);
let min_y = field.into_iter().map(|(_,v)|*v ).fold(0, std::cmp::min);
let mut engine = console_engine::ConsoleEngine::init((min_x.abs()+max_x+1) as u32, (min_y.abs()+max_y+1) as u32, 1).expect("error");

    
    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
        engine.clear_screen(); 
        for coord in field {
        engine.set_pxl((coord.0+min_x.abs()) as i32, (coord.1+min_y.abs()) as i32, pixel::pxl_fg('#', Color::White));
        }
        if engine.is_key_pressed(KeyCode::Char('q')) { // if the user presses 'q' :
            break; // exits app
        }
        engine.draw();



}}