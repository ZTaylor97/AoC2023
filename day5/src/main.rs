use std::fs;

fn main() {
    let document = read_file("./input.txt");
    let lines: Vec<&str> = document.split("\r\n").collect();

    part_one(&lines);
    part_two(&lines);
}

fn part_one(lines: &Vec<&str>) {
    let seeds: Vec<u64> = vec![
        3489262449, 222250568, 2315397239, 327729713, 1284963, 12560465, 1219676803, 10003052,
        291763704, 177898461, 136674754, 107182783, 2917625223, 260345082, 1554280164, 216251358,
        3900312676, 5629667, 494259693, 397354410,
    ];

    let mapping_indices: Vec<((usize, usize), &str)> = vec![
        ((3, 47), "seed_soil"),
        ((50, 69), "soil_fert"),
        ((72, 97), "fert_water"),
        ((100, 138), "water_light"),
        ((141, 184), "light_temp"),
        ((187, 222), "temp_hum"),
        ((225, 235), "hum_location"),
    ];

    let mut locations: Vec<u64> = vec![];

    for seed in seeds {
        println!("seed {seed}");
        let mut val = seed;
        let mut next_val = seed;
        let mut was_mapped = false;
        for (indices, name) in &mapping_indices {
            println!("\t{name}");
            for line in lines[indices.0..=indices.1].iter() {
                let (to_val, from_val, inc) = get_line_nums(line);

                if val >= from_val && val < from_val + inc {
                    was_mapped = true;
                    next_val = to_val + (val - from_val);
                    println!("\t\tval {val}, next_val {next_val} from {from_val}");
                    break;
                }
            }

            if was_mapped {
                val = next_val;
                was_mapped = false;
            } else {
                println!("Was not mapped: val {val} next_val {val}");
            }
        }

        locations.push(val);
    }

    println!("{locations:?}");

    let ans = locations.iter().min().expect("Error finding min value");

    println!("Part 1 ans: {ans}");
}

fn get_line_nums(line: &str) -> (u64, u64, u64) {
    let nums = line
        .split_ascii_whitespace()
        .map(|el| el.parse::<u64>().expect("Error parsing number"))
        .collect::<Vec<_>>();

    (nums[0], nums[1], nums[2])
}

fn part_two(lines: &Vec<&str>) {
    let seeds: Vec<(u64,u64)> = vec![
        (3489262449, 222250568), (2315397239, 327729713), (1284963, 12560465), (1219676803, 10003052),
        (291763704, 177898461), (136674754, 107182783), (2917625223, 260345082), (1554280164, 216251358),
        (3900312676, 5629667), (494259693, 397354410),
    ];

    let mapping_indices: Vec<((usize, usize), &str)> = vec![
        ((3, 47), "seed_soil"),
        ((50, 69), "soil_fert"),
        ((72, 97), "fert_water"),
        ((100, 138), "water_light"),
        ((141, 184), "light_temp"),
        ((187, 222), "temp_hum"),
        ((225, 235), "hum_location"),
    ];

    let mut locations: Vec<u64> = vec![];

    for seed in seeds {
        println!("seed {seed}");
        let mut val = seed;
        let mut next_val = seed;
        let mut was_mapped = false;
        for (indices, name) in &mapping_indices {
            println!("\t{name}");
            for line in lines[indices.0..=indices.1].iter() {
                let (to_val, from_val, inc) = get_line_nums(line);

                if val >= from_val && val < from_val + inc {
                    was_mapped = true;
                    next_val = to_val + (val - from_val);
                    println!("\t\tval {val}, next_val {next_val} from {from_val}");
                    break;
                }
            }

            if was_mapped {
                val = next_val;
                was_mapped = false;
            } else {
                println!("Was not mapped: val {val} next_val {val}");
            }
        }

        locations.push(val);
    }

    println!("{locations:?}");

    let ans = locations.iter().min().expect("Error finding min value");

    println!("Part 1 ans: {ans}");
}

fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Error reading file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let (to, from, inc) = (52, 50, 48);
        let seed = 79;
        assert_eq!(81, to + (seed - from));
    }
}

// let seed_soil_rows: (usize, usize) = (3, 47);
// let soil_fert_rows: (usize, usize) = (50, 69);
// let fert_water_rows: (usize, usize) = (72, 97);
// let water_light_rows: (usize, usize) = (100, 138);
// let light_temp_rows: (usize, usize) = (141, 184);
// let temp_hum_rows: (usize, usize) = (187, 222);
// let hum_loc_rows: (usize, usize) = (225, 235);
