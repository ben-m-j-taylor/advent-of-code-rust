use crate::util::parse_file_into_lines;

pub fn solve() -> (i64, i64) {
    const FILE_PATH: &str = "./input_data/2024/day05/test-input.txt";

    (
        solve_part01(FILE_PATH),
        solve_part02(FILE_PATH),
    )
}

fn solve_part01(file_path: &str) -> i64 {
    let lines = parse_file_into_lines(file_path);

    let (rules, updates) = parse_input(lines);
    
    let mut result: i64 = 0;

    for update in updates {
        if !is_update_safe(&update, &rules) {
            continue;
        }

        result += update[update.len() / 2];
    }

    result
}

fn solve_part02(file_path: &str) -> i64 {
    let lines = parse_file_into_lines(file_path);

    let (rules, updates) = parse_input(lines);
    
    let mut result: i64 = 0;

    for update in updates {
        let mut update_copy = update.clone();

        if is_update_safe(&update_copy, &rules) {
            continue;
        }

        println!("\nIncorrect order = {:?}", update_copy);
        
        // while !is_update_safe(&update_copy, &rules) {
        //     for i in 0..update_copy.len() {
        //         println!("iteration {} - current order = {:?}", i, update_copy);
        // 
        //         if i > 0 {
        //             let possible_prev_numbers: Vec<i64> = get_possible_prev_numbers(update_copy[i], &rules);
        // 
        //             println!("possible_prev_numbers = {:?}", possible_prev_numbers);
        // 
        //             let update_copy_copy_copy = update_copy.clone();
        // 
        //             let prev_allowed_number = &update_copy_copy_copy.iter()
        //                 .find(|x| possible_prev_numbers.contains(x));
        // 
        //             match prev_allowed_number {
        //                 Some(number) => {
        //                     let index_of_prev_allowed_number = update_copy_copy_copy
        //                         .iter()
        //                         .position(|x| x == *number)
        //                         .expect("Couldn't find index of prev allowed number ");
        // 
        //                     update_copy.remove(index_of_prev_allowed_number);
        // 
        //                     update_copy.insert(i-1, **number);
        //                 },
        //                 None => {
        //                     println!("No prev allowed number found");
        //                 }
        //             }
        //         }
        // 
        //         if i < update_copy.len() {
        //             println!("i = {}, number = {}", i, update_copy[i]);
        // 
        //             let possible_next_numbers: Vec<i64> = get_possible_next_numbers(update_copy[i], &rules);
        // 
        //             println!("possible_next_numbers = {:?}", possible_next_numbers);
        // 
        //             let update_copy_copy = update_copy.clone();
        // 
        //             let next_allowed_number = &update_copy_copy.iter()
        //                 .find(|x| possible_next_numbers.contains(x));
        // 
        //             match next_allowed_number {
        //                 Some(number) => {
        //                     let index_of_next_allowed_number = update_copy_copy
        //                         .iter()
        //                         .position(|x| x == *number)
        //                         .expect("Couldn't find index of next allowed number ");
        // 
        //                     update_copy.remove(index_of_next_allowed_number);
        // 
        //                     update_copy.insert(i+1, **number);
        //                 },
        //                 None => {
        //                     println!("Couldn't find next allowed number ");
        //                 }
        //             }
        //         }
        //     }
        // }

        result += update_copy[update_copy.len() / 2];
    }

    result
}

fn parse_input(lines: Vec<String>) -> (Vec<(i64, i64)>, Vec<Vec<i64>>) {
    let mut rules:  Vec<(i64, i64)> = Vec::new();

    let mut updates: Vec<Vec<i64>> = Vec::new();

    let mut adding_updates = false;

    for line in lines {
        if line == "" {
            adding_updates = true;
            continue;
        }

        if !adding_updates {
            let parts = line
                .split("|")
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            rules.push((parts[0], parts[1]));

            continue;
        }

        let parts: Vec<i64> = line.split(",").map(|x| x.parse::<i64>().unwrap()).collect();

        updates.push(parts);
    }

    (rules, updates)
}

fn is_update_safe(update: &Vec<i64>, rules: &Vec<(i64, i64)>) -> bool {
    for (i, number) in update.iter().enumerate() {
        let possible_next_numbers: Vec<i64> = get_possible_next_numbers(*number, &rules);

        let slice = update[i+1..update.len()].to_vec();

        for a in slice {
            let is_safe = possible_next_numbers.iter().any(|x| *x == a);

            if !is_safe {
                return false;
            }
        }
    }

    true
}

fn get_possible_next_numbers(number: i64, rules: &Vec<(i64, i64)>) -> Vec<i64> {
    let possible_next_numbers: Vec<i64> = rules.iter()
        .filter(|(key, _value)| *key == number)
        .map(|(_key, value)| *value)
        .collect();

    possible_next_numbers
}

fn get_possible_prev_numbers(number: i64, rules: &Vec<(i64, i64)>) -> Vec<i64> {
    let possible_prev_numbers: Vec<i64> = rules.iter()
        .filter(|(_key, value)| *value == number)
        .map(|(key, _value)| *key)
        .collect();

    possible_prev_numbers
}
