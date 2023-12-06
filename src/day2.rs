use std::{
    fs::File,
    io::{self, BufRead},
};

fn part1(lines: &[String]) {
    let valid_counts = [12, 13, 14];

    let mut valid_games: Vec<usize> = vec![];
    lines.iter().for_each(|line| {
        let (first, sets) = line.split_once(':').unwrap();

        let game_index = first.split_once(' ').unwrap().1.parse::<usize>().unwrap();

        // start as possible
        let mut is_possible = true;

        sets.split(';').for_each(|set| {
            set.split(", ").for_each(|count_color| {
                let (count_str, color) = count_color.trim().split_once(' ').unwrap();
                let count = count_str.parse::<usize>().unwrap();

                let valid = match color {
                    "red" => valid_counts[0] >= count,
                    "green" => valid_counts[1] >= count,
                    "blue" => valid_counts[2] >= count,
                    _ => panic!("Color not supported: {}", color),
                };

                is_possible = is_possible && valid;
            })
        });

        if is_possible {
            valid_games.push(game_index);
        }
    });

    println!("Result: {}", valid_games.iter().sum::<usize>())
}

fn part2(lines: &[String]) {
    let mut multi_games = vec![];
    lines.iter().for_each(|line| {
        let (_, sets) = line.split_once(':').unwrap();

        let mut red_counts = vec![];
        let mut blue_counts = vec![];
        let mut green_counts = vec![];

        sets.split(';').for_each(|set| {
            set.split(", ").for_each(|count_color| {
                let (count_str, color) = count_color.trim().split_once(' ').unwrap();
                let count = count_str.parse::<usize>().unwrap();

                match color {
                    "red" => {
                        red_counts.push(count);
                    }
                    "green" => {
                        green_counts.push(count);
                    }
                    "blue" => {
                        blue_counts.push(count);
                    }
                    _ => panic!("Color not supported: {}", color),
                };
            })
        });

        multi_games.push(
            red_counts.into_iter().max().unwrap_or_default()
                * blue_counts.into_iter().max().unwrap_or_default()
                * green_counts.into_iter().max().unwrap_or_default(),
        );
    });

    println!("Result: {}", multi_games.iter().sum::<usize>())
}

fn main() {
    let file = File::open("./data/day2.input").expect("file not found!");
    let buf_reader = io::BufReader::new(file);
    let lines: Vec<String> = buf_reader.lines().flatten().collect();

    part1(&lines);
    part2(&lines);
}
