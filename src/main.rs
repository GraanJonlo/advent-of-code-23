use std::fs;

fn main() {
    let file_path = "day01.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut numbers = Vec::new();

    for line in contents.lines() {
        let mut digits = line.chars()
            .filter(|c| c.is_numeric())
            .filter_map(|c| c.to_digit(10));

        if let Some(first) = digits.next() {
            if let Some(last) = digits.last().or(Some(first)){
                numbers.push((first * 10) + last);
            }
        }
    }

    let answer: u32 = numbers.iter().sum();

    println!("Answer is {answer}");
}
