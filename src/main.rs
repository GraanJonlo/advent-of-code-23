use std::fs;

fn main() {
    day_01();
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
    }
}
