use core::num;
use std::{cmp, fs};

fn main() {
    let document = read_file("./input.txt");
    let lines: Vec<&str> = document.split("\r\n").collect();

    part_one(&lines);
    part_two(&lines);
}

fn part_one<'a>(lines: &'a Vec<&'a str>) {
    let symbols: Vec<char> = vec!['/', '@', '*', '%', '#', '=', '+', '&', '-', '$'];

    let valid_cells = get_valid_cells(lines, &symbols);

    let mut sum: u64 = 0;

    let mut grid_nums = lines
        .iter()
        .map(|line| get_nums_from_row(line))
        .collect::<Vec<_>>();

    for row in 0..grid_nums.len() {
        for num_pair in 0..grid_nums[row].len() {
            for col in 0..grid_nums[row][num_pair].1.len() {
                if valid_cells[row][grid_nums[row][num_pair].1[col]] {

                    if grid_nums[row][num_pair].0 != 0 {
                        println!(
                            "row {row} col {} num {}",
                            grid_nums[row][num_pair].1[col], grid_nums[row][num_pair].0
                        );

                        sum += grid_nums[row][num_pair].0 as u64;
                        grid_nums[row][num_pair].0 = 0;
                    }
                }
            }
        }
    }

    println!("{sum}");
}

fn part_two(lines: &Vec<&str>) {
    let symbols: Vec<char> = vec!['*'];

    let valid_cells = get_symbol_cells(lines, '*');

    let grid_nums = lines
        .iter()
        .map(|line| get_nums_from_row(line))
        .collect::<Vec<_>>();


        let mut sum: u64 = 0;

    // Each *
    for (row, col) in valid_cells {

        let mut num_buf: Vec<u64> = vec![];
        // Check each row
        for grid_row in [&grid_nums[row - 1], &grid_nums[row], &grid_nums[row + 1]] {
            // Check each number in each row
            for (num, indices) in grid_row {
                for idx in indices {
                    if *idx == col || *idx == col - 1 || *idx == col + 1 {
                        num_buf.push(*num as u64);
                        break;
                    }
                }
            }
        }

        if num_buf.len() == 2 {
            println!("row {row} col {col} nums = {} {}", num_buf[0], num_buf[1]);
            
            sum += num_buf[0] as u64 * num_buf[1] as u64;
        }

        num_buf.clear();
    }

    println!("{sum}");
}

fn get_symbol_cells(lines: &Vec<&str>, symbol: char) -> Vec<(usize, usize)> {
    let mut output: Vec<(usize, usize)> = vec![];

    for (row, line) in lines.iter().enumerate() {
        let num_cols = line.len();

        let row = row;

        for (col, _) in line.match_indices(symbol) {
            let col = col;

            output.push((row, col));
        }
    }

    output
}

fn get_valid_cells(lines: &Vec<&str>, symbols: &Vec<char>) -> [[bool; 140]; 140] {
    let num_rows = lines.len() as i32;

    // get valid cells to check for numbers
    let mut valid_cells = [[false; 140]; 140];

    for (row, line) in lines.iter().enumerate() {
        let num_cols = line.len() as i32;

        let row = row as i32;

        for sym in symbols {
            for (col, _) in line.match_indices(*sym) {
                let col = col as i32;

                for i in -1..=1 {
                    for j in -1..=1 {
                        if row + i >= 0
                            && col + j >= 0
                            && row + i <= num_rows
                            && col + j <= num_cols
                        {
                            valid_cells[(row + i) as usize][(col + j) as usize] = true;
                            // valid_cells.push(((row + i) as usize, (col + j) as usize));
                        }
                    }
                }
            }
        }
    }
    valid_cells
}

fn get_nums_from_row(line: &str) -> Vec<(u32, Vec<usize>)> {
    let mut num_vec: Vec<(u32, Vec<usize>)> = vec![];
    let mut num_buf: Vec<(u8, usize)> = vec![];
    for (idx, ch) in line.bytes().enumerate() {
        if ch.is_ascii_digit() && idx < 139 {
            num_buf.push((ch, idx));
        } else if ch.is_ascii_digit() && idx == 139 {
            num_buf.push((ch, idx));
            if num_buf.len() > 0 {
                let num = num_buf
                    .iter()
                    .map(|el| el.0 as char)
                    .collect::<String>()
                    .parse::<u32>()
                    .expect("error parsing number");
                let indices = num_buf.iter().map(|el| el.1).collect::<Vec<usize>>();

                num_vec.push((num, indices));
            }

            num_buf.clear();
        } else {
            if num_buf.len() > 0 {
                let num = num_buf
                    .iter()
                    .map(|el| el.0 as char)
                    .collect::<String>()
                    .parse::<u32>()
                    .expect("error parsing number");
                let indices = num_buf.iter().map(|el| el.1).collect::<Vec<usize>>();

                num_vec.push((num, indices));
            }

            num_buf.clear();
        }
    }
    num_vec
}

fn print_valid_cells(valid_cells: [[bool; 140]; 140]) {
    for i in 0..valid_cells.len() {
        for j in 0..valid_cells[i].len() {
            if valid_cells[i][j] {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Error reading file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_values1() {}

    #[test]
    fn get_nums_from_row1() {
        let result = get_nums_from_row("...................305.124................................432..............................................576..313.....514.................").iter().map(|el| el.0).collect::<Vec<u32>>();
        let vec = vec![305, 124, 432, 576, 313, 514];
        assert_eq!(result, vec);
    }

    #[test]
    fn get_nums_from_row2() {
        let result = get_nums_from_row("..690....916.497....................894.440......547$.............596.492......23...............................*...............237.221..786").iter().map(|el| el.0).collect::<Vec<u32>>();
        println!("{result:?}");
        let vec = vec![690, 916, 497, 894, 440, 547, 596, 492, 23, 237, 221, 786];
        assert_eq!(result, vec);
    }
}
