use std::fs;

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
                        sum += u64::from(grid_nums[row][num_pair].0);
                        grid_nums[row][num_pair].0 = 0;
                    }
                }
            }
        }
    }

    println!("Part one ans: {sum}");
}

fn part_two(lines: &Vec<&str>) {
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
                        num_buf.push(u64::from(*num));
                        break;
                    }
                }
            }
        }

        if num_buf.len() == 2 {
            sum += u64::from(num_buf[0]) * u64::from(num_buf[1]);
        }

        num_buf.clear();
    }

    println!("Part two ans: {sum}");
}

/// Get indices of all cells occupied by the input symbol
fn get_symbol_cells(lines: &Vec<&str>, symbol: char) -> Vec<(usize, usize)> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.bytes()
                .enumerate()
                .filter(|(_, ch)| char::from(*ch) == symbol)
                .map(move |(col, _)| (row, col))
        })
        .collect::<Vec<_>>()
}

fn get_valid_cells(lines: &Vec<&str>, symbols: &Vec<char>) -> [[bool; 140]; 140] {
    let num_rows = lines.len() as i32;
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
        if ch.is_ascii_digit() {
            num_buf.push((ch, idx));
        }

        if !ch.is_ascii_digit() || idx == 139 {
            if num_buf.len() > 0 {
                let num = buf_to_num(&num_buf);
                let indices = num_buf.iter().map(|el| el.1).collect::<Vec<usize>>();

                num_vec.push((num, indices));
            }

            num_buf.clear();
        }
    }

    num_vec
}

fn buf_to_num(num_buf: &Vec<(u8, usize)>) -> u32 {
    let num = num_buf
        .iter()
        .map(|el| char::from(el.0))
        .collect::<String>()
        .parse::<u32>()
        .expect("error parsing number");
    num
}

/// Debug function to check grid of cells to check
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
