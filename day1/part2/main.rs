use std::fs;
use std::collections::HashMap;

fn main() {
    let numbers = create_number_word_map();

    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut sum = 0;


    let mut current_word = String::new();
    for line in contents.lines() {

        let mut line_digits = Vec::new();

        for c in line.chars() {
            if c.is_digit(10) {
                line_digits.push(c.to_string());
                current_word.clear();
                continue;
            }

            if !c.is_ascii() {
                continue;
            }

            current_word.push(c);

            if let Some(&number) = numbers.get(current_word.as_str()) {
                line_digits.push(number.to_string());
                current_word.clear();
                continue;
            }

            for i in 0..current_word.len() {
                let sub = &current_word[i..];
                if let Some(&number) = numbers.get(sub) {

                    line_digits.push(number.to_string());
                    current_word.clear();
                    break;
                }
            }
        }

        let first: String = line_digits[0].to_string();
        let last: &str  = &line_digits.pop().unwrap().to_string();
        let fin: String  = format!("{}{}", first, last);

        sum += fin.parse::<i32>().unwrap();
    }

    println!("Sum: {}", sum);
}

fn create_number_word_map() -> HashMap<&'static str, u8> {
    let numbers = [
        ("one", 1), ("two", 2), ("three", 3), ("four", 4),
        ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
    ];

    numbers.iter().cloned().collect()
}

