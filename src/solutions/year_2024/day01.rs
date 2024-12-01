use crate::util::parse_file_into_lines;

pub fn solve() -> (i64, i64) {
    const FILE_PATH: &str = "./input_data/2024/day_01/full-input.txt";
    
    let lines = parse_file_into_lines(FILE_PATH);

    (
        solve_part01(&lines),
        solve_part02(&lines)
    )
}

fn solve_part01(lines: &Vec<String>) -> i64 {
    let (mut first_list, mut second_list) = parse_lines_into_lists(lines);

    first_list.sort();
    second_list.sort();

    let mut result: i64 = 0;

    for i in 0..first_list.len() {
        let a = first_list[i];
        let b = second_list[i];

        if a <= b {
            result += (b - a) as i64;
        } else {
            result += (a - b) as i64;
        }
    }

    result
}

fn solve_part02(lines: &Vec<String>) -> i64 {
    let (first_list, second_list) = parse_lines_into_lists(lines);

    let mut result: i64 = 0;

    for i in 0..first_list.len() {
        let num_from_first_list = first_list[i];

        let number_of_occurrences_second_list = second_list.iter().filter(|&x| *x == num_from_first_list).count() as i32;

        result += (num_from_first_list * number_of_occurrences_second_list) as i64;
    }

    result
}

fn parse_lines_into_lists(lines: &Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();

    for line in lines {
        let split: Vec<&str> = line.split("   ").collect();

        let first_number: i32 = split[0].parse().unwrap();
        let second_number: i32 = split[1].parse().unwrap();

        first_list.push(first_number);
        second_list.push(second_number);
    }

    (first_list, second_list)
}