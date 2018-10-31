use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("day2.txt").unwrap();
    let reader = BufReader::new(f);

    let start = (-2i32, 0i32);
    let solution: String = reader
        .lines()
        .filter_map(|line| line.ok())
        .scan(start, |position, line| {
            *position = line
                .chars()
                .map(|letter| match letter {
                    'U' => (0, -1),
                    'R' => (1, 0),
                    'D' => (0, 1),
                    'L' => (-1, 0),
                    _ => unreachable!(),
                }).fold(*position, |init, direction| {
                    let first = init.0 + direction.0;
                    let second = init.1 + direction.1;
                    if first.abs() + second.abs() > 2 {
                        init
                    } else {
                        (first, second)
                    }
                });
            Some(*position)
        }).map(|position| match position {
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
            _ => unreachable!(),
        }).collect();

    println!("{}", solution);
}
