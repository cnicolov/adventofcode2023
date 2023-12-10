use std::{env::args, fs};

const COORDS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn main() {
    let data = fs::read_to_string("input.txt").unwrap();

    let mut grid = Vec::new();
    // Make the grid
    data.lines().for_each(|line| {
        let mut row = Vec::new();
        line.chars().for_each(|c| {
            row.push(c);
        });
        grid.push(row);
    });

    let mut sum = 0;
    let mut has_part = false;
    let mut n = 0;


    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c.is_digit(10) {
                n = n * 10 + c.to_digit(10).unwrap() as i32;
                for (dx, dy) in COORDS.iter() {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx >= 0 && nx < row.len() as i32 && ny >= 0 && ny < grid.len() as i32 {
                        if !grid[ny as usize][nx as usize].is_digit(10) && grid[ny as usize][nx as usize] != '.' {
                            has_part = true;
                        }
                    }
                }
            } else {
                if has_part {
                    println!("{}", n);
                    sum += n;
                    has_part = false;
                }
                n = 0;
            }
        }
        if has_part {
            println!("{}", n);
            sum += n;
            has_part = false;
        }
        n = 0;
    }

    println!("{}", sum);
}
