use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("./data/day1.input").expect("file not found!");
    let buf_reader = io::BufReader::new(file);

    let numbers = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let subtotal = 0;
    let total = buf_reader
        .lines()
        .flatten()
        .fold(subtotal, |subtotal, line| {
            let mut digits: Vec<char> = vec![];
            // PART 2
            for cursor in 0..line.chars().count() {
                let num = line.chars().nth(cursor).unwrap();
                if num.is_numeric() {
                    digits.push(num)
                }
                for (index, number) in numbers.iter().enumerate() {
                    if line[cursor..].starts_with(number) {
                        digits.push(char::from_digit(index as u32, 10).unwrap())
                    }
                }
            }

            // PART 1
            let first_number = digits.first().unwrap();
            let last_number = digits.iter().last().unwrap();
            let jupla: i32 = (first_number.to_string() + &last_number.to_string())
                .parse()
                .unwrap();

            subtotal + jupla
        });

    println!("Result: {}", total);
}
