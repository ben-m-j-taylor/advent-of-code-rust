use crate::util::parse_file_into_lines;

pub fn solve() -> (i64, i64) {
    const FILE_PATH: &str = "./input_data/2024/day_02/full-input.txt";

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

fn solve_part2(lines: &Vec<String>) -> i64 {
    let mut safe_report_count: i64 = 0;

    for line in lines {
        let numbers: Vec<i16> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();

        let is_array_all_increasing = is_array_all_increasing_and_safe_with_dampener(&numbers);

        let is_array_all_decreasing = is_array_all_decreasing_and_safe_with_dampener(&numbers);

        println!("{:?} - {}", numbers, is_array_all_increasing || is_array_all_decreasing);

        if is_array_all_increasing || is_array_all_decreasing { 
            safe_report_count += 1;
        }
    }

    safe_report_count
}

fn is_array_all_increasing_and_safe_with_dampener(numbers: &Vec<i16>) -> bool {
    let mut dampener_used: bool = false;
    
    let mut i: usize = 0;
    
    while i < numbers.len() {
        if i == 0 {
            i += 1;
            continue;
        }

        let last_number = numbers[i - 1];
        let current_number = numbers[i];

        let last_number_was_larger = last_number > current_number;

        let difference = current_number - last_number;

        let is_unsafe = difference == 0 || difference > 3;

        if !last_number_was_larger && !is_unsafe {
            i += 1;
            continue
        }
        
        if (last_number_was_larger || is_unsafe) && dampener_used {
            return false;
        }
        
        if i + 1 >= numbers.len() {
            i += 1;
            continue;
        }
        
        let next_number = numbers[i + 1];

        let next_number_larger_than_last_number = next_number > last_number;

        let difference_between_next_and_last_number = next_number - last_number;

        let last_number_to_next_number_is_unsafe = difference_between_next_and_last_number == 0 || difference_between_next_and_last_number > 3;

        if next_number_larger_than_last_number && !last_number_to_next_number_is_unsafe {
            i = i + 2;
            dampener_used = true;
        } else {
            return false;
        }
    }

    true
}

fn is_array_all_decreasing_and_safe_with_dampener(numbers: &Vec<i16>) -> bool {
    let mut dampener_used: bool = false;

    let mut i: usize = 0;

    while i < numbers.len() {
        if i == 0 {
            i += 1;
            continue;
        }

        let last_number = numbers[i - 1];
        let current_number = numbers[i];

        let last_number_was_smaller = last_number < current_number;

        let difference = last_number - current_number;

        let is_unsafe = difference == 0 || difference > 3;

        if !last_number_was_smaller && !is_unsafe {
            i += 1;
            continue
        }

        if (last_number_was_smaller || is_unsafe) && dampener_used {
            return false;
        }

        if i + 1 >= numbers.len() {
            i += 1;
            continue;
        }

        let next_number = numbers[i + 1];

        let next_number_smaller_than_last_number = next_number < last_number;

        let difference_between_next_and_last_number = last_number - next_number;

        let last_number_to_next_number_is_unsafe = difference_between_next_and_last_number == 0 || difference_between_next_and_last_number > 3;

        if next_number_smaller_than_last_number && !last_number_to_next_number_is_unsafe {
            i = i + 2;
            dampener_used = true;
        } else {
            return false;
        }
    }

    true
}
