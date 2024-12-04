use crate::util::parse_file_into_lines;

pub fn solve() -> (i64, i64) {
    const FILE_PATH: &str = "./input_data/2024/day02/full-input.txt";

    let lines = parse_file_into_lines(FILE_PATH);

    (
        solve_part1(&lines),
        solve_part2(&lines),
    )
}

fn solve_part1(lines: &Vec<String>) -> i64 {
    let mut safe_report_count: i64 = 0;

    for line in lines {
        let numbers: Vec<i16> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();

        if is_array_all_increasing_and_safe(&numbers) || is_array_all_decreasing_and_safe(&numbers) {
            safe_report_count += 1;
        }
    }

    safe_report_count
}

fn solve_part2(lines: &Vec<String>) -> i64 {
    lines.iter().filter(|x| check_line_is_safe(x)).count() as i64
}

fn check_line_is_safe(line: &String) -> bool {
    let numbers: Vec<i16> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
    
    if check_safety(&numbers) {
        return true;
    }
    
    for i in 0..numbers.len() {
        let mut numbers_copy = numbers.clone();
        
        numbers_copy.remove(i);
        
        if check_safety(&numbers_copy) {
            return true;
        }
    }
    
    false
}

fn check_safety(numbers: &Vec<i16>) -> bool {
    is_array_all_increasing_and_safe(&numbers) || is_array_all_decreasing_and_safe(&numbers)
}

fn is_array_all_increasing_and_safe(numbers: &Vec<i16>) -> bool {
    for i in 0..numbers.len() {
        if i == 0 {
            continue;
        }

        let is_decreasing = numbers[i-1] > numbers[i];

        let difference = numbers[i] - numbers[i-1];

        if is_decreasing || difference == 0 || difference > 3 {
            return false;
        }
    }

    true
}

fn is_array_all_decreasing_and_safe(numbers: &Vec<i16>) -> bool {
    for i in 0..numbers.len() {
        if i == 0 {
            continue;
        }

        let is_increasing = numbers[i-1] < numbers[i];

        let difference = numbers[i-1] - numbers[i];

        if is_increasing || difference == 0 || difference > 3 {
            return false;
        }
    }

    true
}