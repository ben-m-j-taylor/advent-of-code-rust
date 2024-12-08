use crate::util::parse_file_into_lines;

struct InputData {
    test_value: i64,
    calculation_inputs: Vec<i64>
}

pub fn solve() -> (i64, i64) {
    const FILE_PATH: &str = "./input_data/2024/day07/full-input.txt";

    let lines: Vec<String> = parse_file_into_lines(FILE_PATH);

    let input_data: Vec<InputData> = split_lines_into_input_data(lines);

    (
        solve_part01(&input_data),
        solve_part02(input_data),
    )
}

fn split_lines_into_input_data(lines: Vec<String>) -> Vec<InputData> {
    lines
        .iter()
        .map(|x| {
            let first_split = x.split(": ").collect::<Vec<&str>>();

            let test_value: i64 = first_split[0].parse().unwrap();

            let calculation_inputs: Vec<i64> = first_split[1]
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            
            InputData { test_value, calculation_inputs }
        })
        .collect()
}

fn solve_part01(input_data: &Vec<InputData>) -> i64 {
    let mut result: i64 = 0;
    
    for input in input_data {
        let operator_combinations = generate_operator_combinations(input.calculation_inputs.len() - 1, 2);
        
        for combination in operator_combinations {
            if test_combination(input.test_value, &input.calculation_inputs, combination) {
                result += input.test_value;
                break;
            }
        }
    }

    result
}

fn solve_part02(input_data: Vec<InputData>) -> i64 {
    let mut result: i64 = 0;

    for input in input_data {
        let operator_combinations = generate_operator_combinations(input.calculation_inputs.len() - 1, 3);
        
        for combination in operator_combinations {
            if test_combination(input.test_value, &input.calculation_inputs, combination) {
                result += input.test_value;
                break;
            }
        }
    }

    result
}

fn generate_operator_combinations(number_of_calculation_inputs: usize, range_size: u8) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    let mut current = vec![0; number_of_calculation_inputs];

    fn backtrack(pos: usize, n: usize, current: &mut Vec<u8>, result: &mut Vec<Vec<u8>>, range_size: u8) {
        if pos == n {
            result.push(current.clone());
            return;
        }

        for i in 0..range_size {
            current[pos] = i;
            backtrack(pos + 1, n, current, result, range_size);
        }
    }

    backtrack(0, number_of_calculation_inputs, &mut current, &mut result, range_size);
    
    result
}

fn test_combination(test_value: i64, calculation_inputs: &Vec<i64>, operator_combination: Vec<u8>) -> bool {
    let mut result: i64 = calculation_inputs[0];

    let mut i = 0;

    for operator in operator_combination {
        let next_input: i64 = calculation_inputs[i + 1];

        match operator {
            0 => {
                result += next_input;
            },
            1 => {
                result *= next_input;
            },
            2 => {
                let combined: String = result.clone().to_string().to_owned() +
                    next_input.to_string().as_str();

                result = combined.parse::<i64>().unwrap();
            },
            _ => panic!("Unknown operator combinator")
        }

        i += 1;
    }

    result == test_value
}