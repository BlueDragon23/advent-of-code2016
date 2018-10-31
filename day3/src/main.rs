use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("day3.txt").unwrap();
    let reader = BufReader::new(f);
    let lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();
    let count = lines
        // find the triangles
        .chunks(3)
        .flat_map(|lines| {
            let values: Vec<u32> = lines
                .iter()
                .flat_map(|line| line.split_whitespace().map(|x| x.parse::<u32>().unwrap()))
                .collect();

            (0..2).map(move |x| vec![values[x], values[x + 3], values[x + 6]])
        })
        // find the triangles which are valid
        .filter(|triangle| {
            let largest = triangle.iter().max().unwrap();
            *largest < triangle.into_iter().sum::<u32>() - *largest
        }).count();

    println!("{}", count);
}
