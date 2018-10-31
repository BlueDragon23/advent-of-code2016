use std::collections::HashSet;
use std::fs;

fn main() {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let buffer = fs::read_to_string("day1.txt").unwrap();

    let direction = 0i32;
    let position = (0, 0);
    let mut set = HashSet::new();
    set.insert((0, 0));

    let final_position = buffer
        // tokenise
        .split(",")
        .map(|section| section.trim())
        // parse tokens
        .map(|section| {
            let (turn, distance) = section.split_at(1);
            let turn = match turn {
                "R" => 1,
                "L" => -1,
                _ => unreachable!(),
            };
            let distance: i32 = distance.parse().unwrap();
            (turn, distance)
        })
        // find all visited points
        .scan((position, direction), |state, (turn, distance)| {
            let &mut (ref mut position, ref mut direction) = state;
            *direction = (((*direction + turn) % 4) + 4) % 4;

            let direction = directions[*direction as usize];

            let output = Some((*position, distance, direction));

            // adjust *position to be the final value
            position.0 += distance * direction.0;
            position.1 += distance * direction.1;

            output
        }).flat_map(|(position, distance, direction)| {
            (0..distance).map(|x| x + 1).map(move |distance| {
                (
                    position.0 + distance * direction.0,
                    position.1 + distance * direction.1,
                )
            })
        })
        // find duplicates
        .skip_while(|&position| if set.contains(&position) {
            false
        } else {
            set.insert(position);
            true
        })
        .next()
        .expect("Did not find a duplicate");

    println!("{}, {}", final_position.0, final_position.1);
    println!("{}", final_position.0.abs() + final_position.1.abs());
}
