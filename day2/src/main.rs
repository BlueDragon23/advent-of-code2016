use std::cmp::{min,max};
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
    let f = File::open("day2.txt").unwrap();
    let reader = BufReader::new(f);
    
    let mut start = (-2i32, 0i32);
    let mut solution = String::new();
    for maybe_line in reader.lines() {
        if let Ok(line) = maybe_line {
            start = line.chars().fold(start, |init, letter| {
                let direction = match letter {
                    'U' => (0, -1),
                    'R' => (1, 0),
                    'D' => (0, 1),
                    'L' => (-1, 0),
                    _ => {
                        println!("what");
                        (0, 0)
                    },
                };
                let first = init.0 + direction.0;
                let second = init.1 + direction.1;
                if first.abs() + second.abs() > 2 {
                    return init;
                } else {
                    return (first, second)
                }
            });
            solution.push(match start {
                (0, -2) => '1',
                (-1, -1) => '2',
                (0, -1) => '3',
                (1, -1) => '4',
                (-2, 0) => '5',
                (-1, 0) => '6',
                (0, 0) => '7',
                (1, 0) => '8',
                (2, 0) => '9',
                (-1, 1) => 'A',
                (0, 1) => 'B',
                (1, 1) => 'C',
                (0, 2) => 'D',
                _ => {
                    println!("why");
                    '0'
                },
            });
        }
    }
    println!("{}", solution);
    
    
}