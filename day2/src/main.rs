use std::{cmp, fs};

fn main() {
    let document = read_file("./input.txt");
    let lines: Vec<&str> = document.split("\n").collect();

    part_one(&lines);
    part_two(&lines);
}

fn part_one(lines: &Vec<&str>) {
    let (red_max, green_max, blue_max) = (12, 13, 14);

    let sum_score: u64 = lines.iter()
        .enumerate()
        .map(|(idx, line)| (idx, find_colour_max(line)))
        .filter(|(_,(red,green,blue))| !(*red > red_max || *green > green_max || *blue > blue_max))
        .map(|(idx,_)| idx as u64)
        .sum();

    println!("Part 1: {sum_score}");
}

fn part_two(lines: &Vec<&str>) {
    let power_score:u64 = lines.iter()
        .map(|line| {
            let temp = find_colour_max(line);
            (temp.0 * temp.1 * temp.2) as u64
        })
        .sum();

    println!("Part 2: {power_score}");
}

fn find_colour_max(line: &str) -> (i32, i32, i32) {
    let (mut red, mut green, mut blue) = (0, 0, 0);
    let colours = ["red", "green", "blue"];

    let colour_strings = extract_colour_number_pairs(line);

    for col in colour_strings {
        for colour in colours {
            let num = col.split(' ').collect::<Vec<_>>()[0]
                .parse::<i32>()
                .expect("Error extracting number");
            if col.contains(colour) {
                match colour {
                    "red" => red = cmp::max(red, num),
                    "green" => green = cmp::max(green, num),
                    "blue" => blue = cmp::max(blue, num),
                    _ => (),
                }
            }
        }
    }
    (red, green, blue)
}

fn extract_colour_number_pairs(line: &str) -> Vec<&str> {
    let reveal_split = extract_reveals(line);

    let colour_strings: Vec<&str> = reveal_split
        .iter()
        .flat_map(|reveal| reveal.split(','))
        .map(|col| col.trim())
        .collect();
    colour_strings
}

fn extract_reveals(line: &str) -> Vec<&str> {
    let game_split: Vec<&str> = line.split(':').collect();
    game_split[1].trim().split(";").collect()
}

fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Error reading file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_values1() {
        assert_eq!(find_colour_max("Game 1: 13 green, 3 red; 4 red, 9 green, 4 blue; 9 green, 10 red, 2 blue"), (10,13,4));
    }
    #[test]
    fn max_values2() {
        assert_eq!(find_colour_max("Game 43: 7 blue, 16 red, 1 green; 2 red, 6 green, 1 blue; 5 green, 3 red; 5 green, 9 blue, 2 red; 3 red, 9 blue, 4 green; 7 red, 9 blue"), (16,6,9));
    }
}
