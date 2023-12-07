use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

fn part1(lines: &[String]) {
    let mut numbers: Vec<usize> = vec![];
    for (y, line) in lines.iter().enumerate() {
        let mut current_number: String = "".to_string();
        let mut adjacent_symbol = false;
        let mut part_of_number: bool;
        for (x, character) in line.chars().enumerate() {
            part_of_number = character.is_ascii_digit();
            if part_of_number && line.chars().count() - 1 == x {
                current_number += &character.to_string();
                // we're at the end of the line
                // does the number have an adjacent_symbol
                if adjacent_symbol {
                    numbers.push(current_number.parse::<usize>().unwrap());
                }
                // clear state
                current_number = "".to_string();
                adjacent_symbol = false;
                continue;
            }

            match part_of_number {
                true => {
                    // append to current_number
                    current_number += &character.to_string();
                }
                false => {
                    // does the number have an adjacent_symbol
                    if adjacent_symbol {
                        numbers.push(current_number.parse::<usize>().unwrap());
                    }
                    // clear state
                    current_number = "".to_string();
                    adjacent_symbol = false;
                    continue;
                }
            }

            for i in (x as i32) - 1..(x as i32) + 2 {
                if i < 0 || i >= line.chars().count() as i32 {
                    continue;
                }
                for j in (y as i32) - 1..(y as i32) + 2 {
                    if j < 0 || j >= lines.len() as i32 {
                        continue;
                    }

                    let neighbour = lines[j as usize].chars().nth(i as usize).unwrap();
                    let neighbour_is_symbol = !neighbour.is_ascii_digit() && neighbour != '.';
                    if neighbour_is_symbol {
                        // println!("Neighbour: {}", neighbour);
                        adjacent_symbol = true;
                    }
                }
            }
        }
    }

    println!("Result: {}", numbers.iter().sum::<usize>());
}

fn part2(lines: &[String]) {
    let mut possible_gears: HashMap<usize, Vec<usize>> = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        let mut current_number: String = "".to_string();
        let mut adjacent_gears: Vec<usize> = vec![];
        let mut part_of_number: bool;
        for (x, character) in line.chars().enumerate() {
            part_of_number = character.is_ascii_digit();

            if part_of_number {
                current_number += &character.to_string();
            }

            if !part_of_number || line.chars().count() - 1 == x {
                // we're at the end of a number or at the end of the line
                if !adjacent_gears.is_empty() {
                    let found_number = current_number.parse::<usize>().unwrap();
                    adjacent_gears.iter().for_each(|index| {
                        if !possible_gears.contains_key(index) {
                            possible_gears.insert(*index, vec![]);
                        }
                        possible_gears.get_mut(index).unwrap().push(found_number);
                    })
                }
                // clear state
                adjacent_gears = vec![];
                current_number = "".to_string();
                continue;
            }

            if !part_of_number {
                continue;
            }

            for i in (x as i32) - 1..(x as i32) + 2 {
                if i < 0 || i >= line.chars().count() as i32 {
                    continue;
                }
                for j in (y as i32) - 1..(y as i32) + 2 {
                    if j < 0 || j >= lines.len() as i32 {
                        continue;
                    }

                    let neighbour = lines[j as usize].chars().nth(i as usize).unwrap();
                    let index = i as usize + (j as usize * line.chars().count());
                    if neighbour == '*' && !adjacent_gears.contains(&index) {
                        adjacent_gears.push(index)
                    }
                }
            }
        }
    }

    let mut result = 0;
    possible_gears.iter().for_each(|(_gear_location, numbers)| {
        if numbers.len() == 2 {
            result += numbers.iter().product::<usize>();
        }
    });

    println!("Result: {}", result);
}

fn main() {
    let file = File::open("./data/day3.input").expect("file not found!");
    let buf_reader = io::BufReader::new(file);
    let lines: Vec<String> = buf_reader.lines().flatten().collect();

    part1(&lines);
    part2(&lines);
}
