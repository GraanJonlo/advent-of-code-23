use std::fs;
use std::collections::{HashMap, HashSet};

fn main() {
    day_01();
    day_02();
    day_03();
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

    let games: Vec<Game> = parse_lines_day_02(contents.lines());

    day_02_part_1(&games);
    day_02_part_2(&games);
}

fn parse_lines_day_02(lines: std::str::Lines) -> Vec<Game> {
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

struct EnginePart {
    part_number: u32,
    symbol: char,
}

struct Gear {
    part_number1: u32,
    part_number2: u32,
}

fn day_03() {
    let file_path = "day03.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let cells = get_cells(contents);

    let engine_parts = get_engine_parts(&cells);
    let gears = get_gears(&cells);

    day_03_part_1(engine_parts);
    day_03_part_2(gears);
}

fn day_03_part_1(engine_parts: Vec<EnginePart>) {
    let answer: u32 =
        engine_parts
            .iter()
            .map(|part| part.part_number)
            .sum();

    println!("Day 03 part 1 answer is {answer}");
}

fn day_03_part_2(gears: Vec<Gear>) {
    let answer: u32 =
        gears
            .iter()
            .map(|gear| gear.part_number1 * gear.part_number2)
            .sum();

    println!("Day 03 part 2 answer is {answer}");
}

fn get_cells(contents: String) -> Vec<Vec<Option<char>>> {
    let mut cells = Vec::new();

    for line in contents.lines() {
        let mut cell_row = Vec::new();
        for c in line.chars() {
            if c == '.' {
                cell_row.push(None);
            } else {
                cell_row.push(Some(c));
            }
        }
        cells.push(cell_row);
    }

    cells
}

fn get_engine_parts(cells: &Vec<Vec<Option<char>>>) -> Vec<EnginePart> {
    let numbers_and_neighbours = get_numbers_and_neighbours(&cells);

    let mut engine_parts = Vec::new();

    for (part_number, symbol_coords) in numbers_and_neighbours {
        for (y, x) in symbol_coords {
            let symbol: char = cells[y as usize][x as usize].expect("Invalid cell coordinates");

            let engine_part =
                EnginePart { part_number, symbol };

            engine_parts.push(engine_part);
        }
    }

    engine_parts
}

fn get_gears (cells: &Vec<Vec<Option<char>>>) -> Vec<Gear> {
    let numbers_and_neighbours = get_numbers_and_neighbours(&cells);

    let lookup = invert_structure(numbers_and_neighbours);

    let mut gears = Vec::new();

    for y in 0..cells.len() {
        for x in 0..cells[y].len() {
            match cells[y][x] {
                Some('*') => {
                    let part_numbers =
                        lookup.get(&(y as u32,x as u32)).expect("Blah!");

                    if part_numbers.len() == 2 {
                        let gear =
                            Gear { part_number1:part_numbers[0], part_number2:part_numbers[1]};

                        gears.push(gear);
                    }
                },
                _ => (),
            }
        }
    }

    gears
}

fn invert_structure(vec: Vec<(u32, HashSet<(u32, u32)>)>) -> HashMap<(u32, u32), Vec<u32>> {
    let mut map = HashMap::new();

    for (number, coordinates_set) in vec {
        for coordinates in coordinates_set {
            map.entry(coordinates)
                .or_insert_with(Vec::new)
                .push(number);
        }
    }

    map
}

fn get_numbers_and_neighbours(cells: &Vec<Vec<Option<char>>>) -> Vec<(u32, HashSet<(u32, u32)>)> {
    let mut in_number = false;
    let mut num_builder: String = String::new();
    let mut neighbour_builder = HashSet::new();

    let mut result = Vec::new();

    for y in 0..cells.len() {
        for x in 0..cells[y].len() {
            match cells[y][x] {
                None => {
                    if in_number {
                        let new_number: u32 = num_builder.parse().expect("Not a number");
                        result.push((new_number, neighbour_builder.clone()));
                        in_number = false;
                        num_builder = String::new();
                        neighbour_builder = HashSet::new();
                    }
                }
                Some(c) => {
                    if c.is_numeric() {
                        if in_number {
                            num_builder.push(c);
                        } else {
                            in_number = true;
                            num_builder.push(c);
                        }
                        let neighbours = neighbour_symbols((y as u32, x as u32), cells);
                        neighbour_builder.extend(neighbours);
                    } else {
                        if in_number {
                            let new_number: u32 = num_builder.parse().expect("Not a number");
                            result.push((new_number, neighbour_builder.clone()));
                            in_number = false;
                            num_builder = String::new();
                            neighbour_builder = HashSet::new();
                        }
                    }
                }
            }
        }
    }

    result
}

fn neighbour_symbols(cell: (u32, u32), cells: &Vec<Vec<Option<char>>>) -> Vec<(u32, u32)> {
    let potentials = potential_neighbours(cell);

    potentials.iter()
        .filter(|&&(y, x)| {
            cells.get(y as usize)
                .and_then(|row| row.get(x as usize))
                .map_or(false, |cell| match cell {
                    Some(c) => !c.is_numeric(),
                    _ => false,
                })
        })
        .cloned()
        .collect()
}

fn potential_neighbours(cell: (u32, u32)) -> Vec<(u32, u32)> {
    let above = potential_neighbours_above(cell);
    let same_row = potential_neighbours_same_row(cell);
    let below = potential_neighbours_below(cell);

    let mut neighbours = Vec::new();

    neighbours.extend(above);
    neighbours.extend(same_row);
    neighbours.extend(below);

    sort_cells(&mut neighbours);

    neighbours
}

fn potential_neighbours_above((y, x): (u32, u32)) -> Vec<(u32, u32)> {
    if y == 0 {
        return Vec::with_capacity(0);
    }

    let mut list = Vec::with_capacity(3);

    if x > 0 {
        list.push((y - 1, x - 1));
    }
    list.push((y - 1, x));
    list.push((y - 1, x + 1));

    list
}

fn potential_neighbours_same_row((y, x): (u32, u32)) -> Vec<(u32, u32)> {
    let mut list = Vec::with_capacity(2);

    if x > 0 {
        list.push((y, x - 1));
    }

    list.push((y, x + 1));

    list
}

fn potential_neighbours_below((y, x): (u32, u32)) -> Vec<(u32, u32)> {
    let mut list = Vec::with_capacity(3);

    if x > 0 {
        list.push((y + 1, x - 1));
    }

    list.push((y + 1, x));
    list.push((y + 1, x + 1));

    list
}

fn sort_cells(neighbours: &mut Vec<(u32, u32)>) {
    neighbours.sort_by(|(y1, x1), (y2, x2)| {
        y1.cmp(&y2).then(x1.cmp(&x2))
    });
}
