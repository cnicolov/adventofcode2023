use std::fs;
const RED: i32 = 12;
const GREEN: i32 = 13;
const BLUE: i32 = 14;

struct Roll {
    red: i32,
    green: i32,
    blue: i32,
}

struct Game {
    id: i32,

    rolls: Vec<Roll>,
}

fn main() {
    let mut sum = 0;
    let mut sum_min = 0;
    let content = fs::read_to_string("./input.txt")
        .expect("Failed to read file");
    content.lines().for_each(|line| {
        println!("{}", line);
        let game = line_to_game(line);
        sum_min += sum_rolls_per_game(&game);
        if game_possible(&game) {
            sum += game.id;
        }
    });
    println!("Part1: {}", sum);
    println!("Part2: {}", sum_min);
}

fn game_possible(game: &Game) -> bool {
    for roll in &game.rolls {
        if roll.red > RED || roll.green > GREEN || roll.blue > BLUE {
            return false;
        }
    }
    true
}

fn line_to_game(line: &str) -> Game {
    Game { id: parse_game_id(line), rolls: parse_rolls(line) }
}

fn parse_rolls(line: &str) -> Vec<Roll> {

    let mut rolls = Vec::new();
    let rolls_str = line.split(":").collect::<Vec<&str>>()[1];
    rolls_str.split(";").for_each(|roll| {
        roll.split(",").for_each(|ball| {
            let pair = ball.trim().split(" ").collect::<Vec<&str>>();
            let count = pair[0].trim().parse::<i32>().unwrap();
            let color = pair[1];
            match color {
                "red" => rolls.push(Roll { red: count, green: 0, blue: 0 }),
                "green" => rolls.push(Roll { red: 0, green: count, blue: 0 }),
                "blue" => rolls.push(Roll { red: 0, green: 0, blue: count }),
                _ => panic!("Invalid color"),
            }
        });

    });
    rolls
}

fn parse_game_id(line: &str) -> i32 {
    line.split(":")
        .collect::<Vec<&str>>()[0]
        .split(" ")
        .collect::<Vec<&str>>()[1]
        .parse::<i32>()
        .unwrap()
}

fn sum_rolls_per_game(game: &Game) -> i32 {
    let mut reds = Vec::new();
    let mut greens = Vec::new();
    let mut blues = Vec::new();
    for roll in &game.rolls {
        reds.push(roll.red);
        blues.push(roll.blue);
        greens.push(roll.green);
    }
    let max_red = max_number(&reds).unwrap();
    let max_green = max_number(&greens).unwrap();
    let max_blue = max_number(&blues).unwrap();

    println!("{}", max_red * max_green * max_blue);
    max_red * max_green * max_blue
}

fn max_number(numbers: &[i32]) -> Option<i32> {
    let mut large = numbers.get(0)?;
    for number in numbers {
        if large < number {
            large = number
        }
    }

    Some(*large)
}
