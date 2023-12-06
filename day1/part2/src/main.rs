use std::fs;

fn main() {

    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut total_sum = 0;

    contents.lines().for_each(|line| {

        let line_chars = line
            .replace("one", "o1ne")
            .replace("two", "t2wo")
            .replace("three", "t3hree")
            .replace("four", "f4our")
            .replace("five", "f5ive")
            .replace("six", "s6ix")
            .replace("seven", "s7even")
            .replace("eight", "e8ight")
            .replace("nine", "n9ine")
            .chars()
            .filter(|c| c.is_digit(10)).collect::<Vec<char>>();
        let res_str = format!("{}{}", line_chars[0], line_chars[line_chars.len() - 1]);
        total_sum += res_str.parse::<i32>().unwrap();
    });
    print!("Total sum: {}", total_sum);
}
