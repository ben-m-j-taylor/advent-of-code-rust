use regex::Regex;
use crate::util::read_file_to_string;

pub fn solve() -> (i64, i64) {
    const FILE_PATH: &str = "./input_data/2024/day_03/test-input.txt";

    let file_contents = read_file_to_string(FILE_PATH);

    (
        solve_part01(&file_contents),
        solve_part02(&file_contents),
    )
}

fn solve_part01(file_contents: &String) -> i64 {
    let mut total: i64 = 0;

    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    for (_, [num1_as_string, num2_as_string]) in re.captures_iter(file_contents).map(|c| c.extract()) {
        let num1: i64 = num1_as_string.parse().unwrap();
        let num2: i64 = num2_as_string.parse().unwrap();

        total += num1 * num2;
    }

    total
}

fn solve_part02(file_contents: &String) -> i64 {
    let mut total: i64 = 0;

    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|don't\(\)|do\(\)").unwrap();

    let mut doing: bool = true;

    for capture in re.captures_iter(file_contents) {
        match capture.get(0).unwrap().as_str() {
            "don't()" => doing = false,
            "do()" => doing = true,
            _ => {
                if doing {
                    let num1: i64 = capture.get(1).unwrap().as_str().parse().unwrap();
                    let num2: i64 = capture.get(2).unwrap().as_str().parse().unwrap();

                    total += num1 * num2;
                }
            }
        }
    }

    total
}