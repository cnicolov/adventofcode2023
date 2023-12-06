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
    let content = fs::read_to_string("./input.txt")
        .expect("Failed to read file");
    content.lines().for_each(|line| {
        println!("{}", line);
        let game = line_to_game(line);
        if game_possible(&game) {
            sum += game.id;
        }
    });
    println!("{}", sum);
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
