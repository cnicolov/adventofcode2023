use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Something went wrong reading the file");
    let lines = contents.lines();

    let mut sum = 0;
    for line in lines {
        let mut line_digits = Vec::new();
        line.chars().for_each(|c| {
            if c.is_digit(10) {
                line_digits.push(c);
            }
        });

        let first: String = line_digits[0].to_string();
        let last: &str  = &line_digits.pop().unwrap().to_string();
        let fin: String  = format!("{}{}", first, last);

        sum += fin.parse::<i32>().unwrap();
    }

    print!("{}", sum);

}
