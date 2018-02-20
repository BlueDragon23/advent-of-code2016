use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
	let f = File::open("day3.txt").unwrap();
	let reader = BufReader::new(f);

	let count = reader.lines().fold(0u32, |acc, line| {
        let values: Vec<u32> = line.unwrap().split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect();
        let largest = values.iter().max().unwrap();
        if largest >= &(values.iter().sum::<u32>() - largest) {
            return acc;
        } else {
            return acc + 1;
        }
    });
    
    println!("{}", count);
}