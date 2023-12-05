use std::fs;

fn main() {
    let document = read_file("./input.txt");
    let lines: Vec<&str> = document.split("\r\n").collect();

    part_one(&lines);
    part_two(&lines);
}

fn part_one(lines: &Vec<&str>) {
}

fn part_two(lines: &Vec<&str>) {
}



fn read_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("Error reading file")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
    }
}
