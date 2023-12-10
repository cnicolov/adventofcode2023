use std::{collections::HashSet, fs};

fn main() {
    let data = fs::read_to_string("input.txt").expect("Unable to read file");

    let mut result = 0;

    for line in data.lines() {
        #[allow(unused_assignments)]
        let l = line.split(":").collect::<Vec<&str>>();
        let cards_and_numbers = l[1].split(" | ").collect::<Vec<&str>>();
        let cards = cards_and_numbers[0]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<HashSet<i32>>();
        let numbers = cards_and_numbers[1]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<HashSet<i32>>();
        let intersection = cards.intersection(&numbers);
        let count = intersection.count();
        let mut points = 0;

        if count == 0 {
            continue;
        }

        for i in 0..count {
            if i == 0 {
                points = 1;
            } else {
                points *= 2;
            }
        }

        println!("{} {}", line, points);

        result += points;
    }

    println!("{}", result);
}
