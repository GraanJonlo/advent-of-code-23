use std::fs;
use std::str::Lines;

fn main() {
    day_01();
    day_02();
}

fn day_01() {
    let file_path = "day01.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    day_01_part_1(&contents);
    day_01_part_2(&contents);
}

fn day_01_part_1(contents: &String) {
    let mut numbers = Vec::new();

    for line in contents.lines() {
        let mut digits = line.chars()
            .filter(|c| c.is_numeric())
            .filter_map(|c| c.to_digit(10));

        if let Some(first) = digits.next() {
            if let Some(last) = digits.last().or(Some(first)) {
                numbers.push((first * 10) + last);
            }
        }
    }

    let answer: u32 = numbers.iter().sum();

    println!("Day 01 part 1 answer is {answer}");
}

fn day_01_part_2(contents: &String) {
    let mut numbers = Vec::new();

    for line in contents.lines() {
        let digits = parse_to_numbers(line, Vec::new());

        if let (Some(first), Some(last)) = (digits.first(), digits.last()) {
            numbers.push(first * 10 + last);
        }
    }

    let answer: u32 = numbers.iter().sum();

    println!("Day 01 part 2 answer is {answer}");
}

fn parse_to_numbers(input: &str, mut acc: Vec<u32>) -> Vec<u32> {
    let mappings = [
        ("one", 1), ("1", 1),
        ("two", 2), ("2", 2),
        ("three", 3), ("3", 3),
        ("four", 4), ("4", 4),
        ("five", 5), ("5", 5),
        ("six", 6), ("6", 6),
        ("seven", 7), ("7", 7),
        ("eight", 8), ("8", 8),
        ("nine", 9), ("9", 9),
    ];

    return if input.is_empty() {
        acc
    } else {
        if let Some(&(_, num)) =
            mappings.iter().find(|(word, _)| input.starts_with(word)) {
            acc.push(num);
        }

        parse_to_numbers(&input[1..], acc)
    };
}

#[derive(Debug, PartialEq)]
enum CubeColour {
    Red,
    Green,
    Blue,
}

#[derive(Debug, PartialEq)]
struct Hand {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    hands: Vec<Hand>,
}

fn day_02() {
    let file_path = "day02.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let games: Vec<Game> = parse_lines(contents.lines());

    day_02_part_1(&games);
    day_02_part_2(&games);
}

fn parse_lines(lines: Lines) -> Vec<Game> {
    lines.filter_map(parse_line).collect()
}

fn parse_line(line: &str) -> Option<Game> {
    let mut parts = line.split(':');
    let id_part = parts.next()?.trim();
    let hands_parts = parts.next()?.trim().split(';');

    let id: u32 = id_part.split_whitespace().last()?.parse().ok()?;

    let hands = hands_parts
        .map(parse_hand).collect();

    Some(Game { id, hands })
}

fn parse_hand(hand_data: &str) -> Hand {
    let cubes = hand_data.trim().split(',')
        .filter_map(|cube| {
            let mut parts = cube.trim().split_whitespace();
            let count: u32 = parts.next()?.parse().ok()?;
            let colour: CubeColour = parts.next().and_then(parse_cube_colour)?;
            Some((colour, count))
        }).collect::<Vec<(CubeColour, u32)>>();

    let red_count = cubes.iter()
        .filter(|(colour, _)| *colour == CubeColour::Red)
        .map(|(_, count)| count)
        .sum();

    let green_count = cubes.iter()
        .filter(|(colour, _)| *colour == CubeColour::Green)
        .map(|(_, count)| count)
        .sum();

    let blue_count = cubes.iter()
        .filter(|(colour, _)| *colour == CubeColour::Blue)
        .map(|(_, count)| count)
        .sum();

    Hand { red: red_count, green: green_count, blue: blue_count }
}

fn parse_cube_colour(s: &str) -> Option<CubeColour> {
    match s {
        "red" => Some(CubeColour::Red),
        "green" => Some(CubeColour::Green),
        "blue" => Some(CubeColour::Blue),
        _ => None,
    }
}

fn day_02_part_1(games: &Vec<Game>) {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;

    let answer: u32 =
        games.iter()
            .filter(|game| game.hands.iter()
                .all(|hand|
                    hand.red <= max_red &&
                        hand.green <= max_green &&
                        hand.blue <= max_blue))
            .map(|game| game.id)
            .sum();

    println!("Day 02 part 1 answer is {answer}");
}

fn day_02_part_2(games: &Vec<Game>) {
    let mut answer = 0;

    for game in games {
        let most_red_seen =
            game.hands.iter().map(|hand| hand.red).max().unwrap_or(0);

        let most_green_seen =
            game.hands.iter().map(|hand| hand.green).max().unwrap_or(0);

        let most_blue_seen =
            game.hands.iter().map(|hand| hand.blue).max().unwrap_or(0);

        answer += most_red_seen * most_green_seen * most_blue_seen;
    }

    println!("Day 02 part 2 answer is {answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_hand_data() {
        let hand_data = "1 green, 3 red, 6 blue";
        let expected_hand = Hand { red: 3, green: 1, blue: 6 };
        assert_eq!(parse_hand(hand_data), expected_hand);
    }

    #[test]
    fn it_parses_red_cube_colour() {
        let colour = "red";

        let result = parse_cube_colour(colour);

        assert_eq!(Some(CubeColour::Red), result);
    }

    #[test]
    fn it_parses_green_cube_colour() {
        let colour = "green";

        let result = parse_cube_colour(colour);

        assert_eq!(Some(CubeColour::Green), result);
    }

    #[test]
    fn it_parses_blue_cube_colour() {
        let colour = "blue";

        let result = parse_cube_colour(colour);

        assert_eq!(Some(CubeColour::Blue), result);
    }
}
