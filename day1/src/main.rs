use std::{cmp, fs};

fn main() {
    let document = read_file("./input.txt");
    let mut lines = document.split_ascii_whitespace();

    let mut sum_part_one: u64 = 0;
    let mut sum_part_two: u64 = 0;
    for line in lines {
        sum_part_one += find_digits_1(line);
        sum_part_two += find_digits_2(line);
    }

    println!("Part 1 Answer: {sum_part_one}\nPart 2 Answer: {sum_part_two}")
}

fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Error reading file")
}

/// Part one: Extract first and last digits from line to create a number
fn find_digits_1(line: &str) -> u64 {
    let digit_indices = find_digits_in_line(line);

    u64::from(
        digit_indices.first().expect("Error: No digits found").1 * 10
            + digit_indices.last().expect("Error: No Digits found").1,
    )
}

/// Part two: Extract first and last digits, including words that spell numbers, to create a number
fn find_digits_2(line: &str) -> u64 {
    let digit_indices = find_digits_in_line(line);
    let word_indices = find_numbers_as_words_in_line(line);

    let only_one = (digit_indices.len() + word_indices.len()) == 1;

    let mut first_num: u32 = 0;
    let mut last_num: u32 = 0;

    let digit_first = digit_indices.first();
    let digit_last = digit_indices.last();

    let word_first = word_indices.iter().min_by(|x, y| x.0.cmp(&y.0));
    let word_last = word_indices.iter().max_by(|x, y| x.0.cmp(&y.0));

    if let Some(digit_first_idx) = digit_first {
        if let Some(word_first_idx) = word_first {
            first_num = std::cmp::min_by(word_first_idx, digit_first_idx, |x, y| x.0.cmp(&y.0)).1;
            last_num = std::cmp::max_by(
                word_last.expect("Error"),
                digit_last.expect("Error"),
                |x, y| x.0.cmp(&y.0),
            )
            .1;
        } else {
            first_num = digit_first.expect("Error: word_indices empty").1;
            last_num = digit_last.expect("Error: word_indices empty").1;
        }
    } else {
        first_num = word_first.expect("Error: word_indices empty").1;
        last_num = word_last.expect("Error: word_indices empty").1;
    }

    if only_one {
        last_num = first_num;
    }

    u64::from(first_num * 10 + last_num)
}

fn find_digits_in_line(line: &str) -> Vec<(usize, u32)> {
    line.as_bytes()
        .iter()
        .enumerate()
        .filter_map(|(idx, ch)| {
            if ch.is_ascii_digit() {
                Some((idx, (*ch) as u32 - 48))
            } else {
                None
            }
        })
        .collect()
}

fn find_numbers_as_words_in_line(line: &str) -> Vec<(usize, u32)> {
    let digits_as_words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    digits_as_words
        .iter()
        .enumerate()
        .flat_map(|(digit, digit_word)| {
            line.match_indices(digit_word)
                .map(move |(line_idx, _)| (line_idx, digit as u32))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_one_digit() {
        assert_eq!(find_digits_1("treb7uchet"), 77);
    }
    #[test]
    fn part_one_two_digits() {
        assert_eq!(find_digits_1("1abc2"), 12);
    }
    #[test]
    fn part_one_multiple_digits() {
        assert_eq!(find_digits_1("a1b2c3d4e5f"), 15);
    }
    #[test]
    fn part_two_one_digit() {
        assert_eq!(find_digits_2("treb7uchet"), 77);
    }
    #[test]
    fn part_two_two_digits() {
        assert_eq!(find_digits_2("1abc2"), 12);
    }
    #[test]
    fn part_two_multiple_digits() {
        assert_eq!(find_digits_2("a1b2c3d4e5f"), 15);
    }
    #[test]
    fn part_two_multiple_digits_words() {
        assert_eq!(find_digits_2("two1nine"), 29);
    }
    #[test]
    fn part_two_multiple_digits_words1() {
        assert_eq!(find_digits_2("eightwothree"), 83);
    }
    #[test]
    fn part_two_multiple_digits_words2() {
        assert_eq!(find_digits_2("abcone2threexyz"), 13);
    }
    #[test]
    fn part_two_multiple_digits_words3() {
        assert_eq!(find_digits_2("7pqrstsixteen"), 76);
    }

    #[test]
    fn part_two_multiple_digits_words4() {
        assert_eq!(find_digits_2("one7sixninesix"), 16);
    }
}
