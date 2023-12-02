use std::{cmp, fs};

fn main() {
    let document = read_file("./input.txt");
    let lines = document.split("\n");

    let colours = ["red", "green", "blue"];

    let (red_max,green_max, blue_max) = (12,13,14);

    let mut sum_score:u64 = 0;
    let mut power_score:u64 = 0;

    for (idx, mut line) in lines.enumerate() {
        line = line.trim();

        let game_split: Vec<&str> = line.split(':').collect();

        let reveal_split: Vec<&str> = game_split[1].trim().split(";").collect();

        let (mut red, mut green, mut blue) = (0, 0, 0);
        

        for reveal in reveal_split {
            let colour_split: Vec<&str> = reveal.split(",").collect();

            for colour_str in colour_split {
                let num_split: Vec<&str> = colour_str.trim_start().split(' ').collect();
                let num = num_split[0]
                    .parse::<i32>()
                    .expect("Error extracting number");

                for colour in colours {
                    if colour_str.contains(colour) {
                        match colour {
                            "red" => red = cmp::max(red, num),
                            "green" => green = cmp::max(green, num),
                            "blue" => blue = cmp::max(blue, num),
                            _ => (),
                        }
                    }
                }
            }
        }

        if !(red > red_max || green > green_max || blue > blue_max) {
            sum_score += idx as u64 + 1;
        }
        power_score += (red*green*blue) as u64;


    }

    println!("Part 1: {sum_score}\nPart 2: {power_score}");
}

fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Error reading file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {}
}
