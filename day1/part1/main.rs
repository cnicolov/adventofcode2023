use std::fs;

fn is_digit(c: &str) -> bool {
    match c {
        "0" => true,
        "1" => true,
        "2" => true,
        "3" => true,
        "4" => true,
        "5" => true,
        "6" => true,
        "7" => true,
        "8" => true,
        "9" => true,
        _ => false,
    }
}

fn main() {
    let contents = fs::read_to_string("./input.txt").expect("Something went wrong reading the file");
    let lines = contents.lines();

    let mut sum = 0;
    for line in lines {
        let mut line_digits = Vec::new();
        line.chars().for_each(|c| {
            if is_digit(&c.to_string()) {
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
