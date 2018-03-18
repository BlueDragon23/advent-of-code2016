use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {
	let f = File::open("day3.txt").unwrap();
	let reader = BufReader::new(f);
    let lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();
	let count = lines.chunks(3).fold(0u32, |mut acc, line| {
        let values: Vec<u32> = line.iter().fold(Vec::new(), |mut v, line| {
            v.extend(line.split_whitespace().map(|x| x.parse::<u32>().unwrap()));
            v
        });
        
        for x in 0..2 {
            let mut triangle = Vec::new();
            triangle.extend([values[x], values[x+3], values[x+6]].iter());
            let largest = triangle.iter().max().unwrap();
            if largest < &(triangle.iter().sum::<u32>() - largest) {
                acc += 1;
            }
        }
        acc
    });
    
    println!("{}", count);
}