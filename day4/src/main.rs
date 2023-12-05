use std::{collections::VecDeque, fs};
use rayon::prelude::*;

fn main() {
    let document = read_file("./input.txt");
    let lines: Vec<&str> = document.split("\r\n").collect();

    part_one(&lines);
    part_two_par(&lines);
}

fn part_one(lines: &Vec<&str>) {
    let sum = lines
        .iter()
        .map(|line| {
            let count = get_win_count(line);
            match count {
                0 => 0,
                1 => 1,
                _ => u64::pow(2, count - 1),
            }
        })
        .fold(0, |acc, el| acc + el);

    println!("Part one ans: {sum}");
}

fn part_two(lines: &Vec<&str>) {
    let mut sum: u64 = 0;
    
    for idx in 0..lines.len() {
        
    let mut copies: VecDeque<usize> = VecDeque::new();
        copies.push_back(idx);

        while copies.len() > 0 {
            sum += 1;
            let copy_idx = copies.pop_front().expect("Error, queue empty");

            let win_count = get_win_count(lines[copy_idx]);

            for win_idx in 1..=win_count {
                copies.push_back(copy_idx + win_idx as usize);
            }
        }

        println!("finished a loop");
    }

    println!("Part two ans: {sum}");
}

/// This approach makes program execution ~3 times as fast on my 6 core laptop
fn part_two_par(lines: &Vec<&str>) {
    let sum: u64 = (0..lines.len())
        .into_par_iter()
        .map(|idx| {
            let mut copies: VecDeque<usize> = VecDeque::new();
            copies.push_back(idx);

            let mut inner_sum = 0;
            while copies.len() > 0 {
                inner_sum += 1;
                let copy_idx = copies.pop_front().expect("Error, queue empty");

                let win_count = get_win_count(lines[copy_idx]);

                for win_idx in 1..=win_count {
                    copies.push_back(copy_idx + win_idx as usize);
                }
            }

            inner_sum
        })
        .sum();

    println!("Part two ans: {sum}");
}

fn separate_numbers(line: &str) -> (Vec<u64>, Vec<u64>) {
    let scores = line.split(':').collect::<Vec<_>>()[1]
        .split('|')
        .collect::<Vec<_>>();

    let winners: Vec<u64> = scores[0]
        .trim()
        .split_ascii_whitespace()
        .map(|num_str| num_str.parse::<u64>().expect("Error parsing number"))
        .collect();
    let numbers: Vec<u64> = scores[1]
        .trim()
        .split_ascii_whitespace()
        .map(|num_str| num_str.parse::<u64>().expect("Error parsing number"))
        .collect();

    (winners, numbers)
}

fn get_win_count(line: &str) -> u32 {
    let (winners, numbers) = separate_numbers(line);

    let mut count = 0;
    for winner in &winners {
        for number in &numbers {
            if number == winner {
                count += 1;
            }
        }
    }

    count
}

fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Error reading file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbers1() {
        let (winners, losers) =
            separate_numbers("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(winners, vec![41, 48, 83, 86, 17]);
        assert_eq!(losers, vec![83, 86, 6, 31, 17, 9, 48, 53]);
    }

    #[test]
    fn game_score1() {
        assert_eq!(
            get_game_score("Card 1: 41 48 83 86 17 | 83 86  6 31 17 9 48 53"),
            8
        );
    }
    #[test]
    fn game_score2() {
        assert_eq!(
            get_game_score("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
            2
        );
    }
    #[test]
    fn game_score3() {
        assert_eq!(
            get_game_score("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            2
        );
    }
    #[test]
    fn game_score4() {
        assert_eq!(
            get_game_score("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            1
        );
    }
    #[test]
    fn game_score5() {
        assert_eq!(
            get_game_score("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            0
        );
    }
    #[test]
    fn game_score6() {
        assert_eq!(
            get_game_score("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
            0
        );
    }
}
