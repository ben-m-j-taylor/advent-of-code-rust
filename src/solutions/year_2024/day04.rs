use crate::util::parse_file_into_lines;

pub fn solve() -> (i64, i64) {
    const FILE_PATH: &str = "./input_data/2024/day04/full-input.txt";

    (
        solve_part01(FILE_PATH),
        solve_part02(FILE_PATH),
    )
}

fn solve_part01(file_path: &str) -> i64 {
    let lines: Vec<String> = parse_file_into_lines(file_path);

    let letter_rows = split_lines_into_rows_of_letters(lines);

    let mut instances: i64 = 0;

    for (i, letter_row) in letter_rows.iter().enumerate() {
        for (j, letter) in letter_row.iter().enumerate() {
            if letter == "X" {
                instances += scan_for_xmas_instances(&letter_rows, i, j)
            }
        }
    }

    instances
}

fn split_lines_into_rows_of_letters(lines: Vec<String>) -> Vec<Vec<String>> {
    lines.iter()
        .map(|x| x.chars()
            .map(|x1| x1.to_string())
            .collect::<Vec<String>>())
        .collect::<Vec<Vec<String>>>()
}

fn scan_for_xmas_instances(letter_rows: &Vec<Vec<String>>, row_index: usize, letter_index: usize) -> i64 {
    let mut instances: i64 = 0;

    let has_enough_letters_on_row_after_current_letter = letter_index + 4 <= letter_rows[row_index].len();

    let has_enough_letters_on_row_before_current_letter = letter_index >= 3;

    let has_enough_rows_before_the_current_row = row_index >= 3;

    let has_enough_rows_after_the_current_row = row_index + 4 <= letter_rows.len();

    if has_enough_letters_on_row_after_current_letter {
        let letter_slice = &letter_rows[row_index][letter_index..letter_index + 4];

        if letter_sequence_is_xmas(&letter_slice[0], &letter_slice[1], &letter_slice[2], &letter_slice[3]) {
            instances += 1;
        }
    }

    if has_enough_letters_on_row_before_current_letter {
        let letter_slice = &letter_rows[row_index][letter_index - 3..letter_index + 1];

        if letter_sequence_is_xmas(&letter_slice[3], &letter_slice[2], &letter_slice[1], &letter_slice[0]) {
            instances += 1;
        }
    }

    if has_enough_rows_before_the_current_row {
        let letter1 = &letter_rows[row_index][letter_index];
        let letter2 = &letter_rows[row_index - 1][letter_index];
        let letter3 = &letter_rows[row_index - 2][letter_index];
        let letter4 = &letter_rows[row_index - 3][letter_index];

        if letter_sequence_is_xmas(letter1, letter2, letter3, letter4) {
            instances += 1;
        }
    }

    if has_enough_rows_after_the_current_row {
        let letter1 = &letter_rows[row_index][letter_index];
        let letter2 = &letter_rows[row_index + 1][letter_index];
        let letter3 = &letter_rows[row_index + 2][letter_index];
        let letter4 = &letter_rows[row_index + 3][letter_index];

        if letter_sequence_is_xmas(letter1, letter2, letter3, letter4) {
            instances += 1;
        }
    }

    if has_enough_rows_before_the_current_row && has_enough_letters_on_row_after_current_letter {
        let letter1 = &letter_rows[row_index][letter_index];
        let letter2 = &letter_rows[row_index - 1][letter_index + 1];
        let letter3 = &letter_rows[row_index - 2][letter_index + 2];
        let letter4 = &letter_rows[row_index - 3][letter_index + 3];

        if letter_sequence_is_xmas(letter1, letter2, letter3, letter4) {
            instances += 1;
        }
    }

    if has_enough_rows_after_the_current_row && has_enough_letters_on_row_after_current_letter {
        let letter1 = &letter_rows[row_index][letter_index];
        let letter2 = &letter_rows[row_index + 1][letter_index + 1];
        let letter3 = &letter_rows[row_index + 2][letter_index + 2];
        let letter4 = &letter_rows[row_index + 3][letter_index + 3];

        if letter_sequence_is_xmas(letter1, letter2, letter3, letter4) {
            instances += 1;
        }
    }

    if has_enough_rows_after_the_current_row && has_enough_letters_on_row_before_current_letter {
        let letter1 = &letter_rows[row_index][letter_index];
        let letter2 = &letter_rows[row_index + 1][letter_index - 1];
        let letter3 = &letter_rows[row_index + 2][letter_index - 2];
        let letter4 = &letter_rows[row_index + 3][letter_index - 3];

        if letter_sequence_is_xmas(letter1, letter2, letter3, letter4) {
            instances += 1;
        }
    }

    if has_enough_rows_before_the_current_row && has_enough_letters_on_row_before_current_letter {
        let letter1 = &letter_rows[row_index][letter_index];
        let letter2 = &letter_rows[row_index - 1][letter_index - 1];
        let letter3 = &letter_rows[row_index - 2][letter_index - 2];
        let letter4 = &letter_rows[row_index - 3][letter_index - 3];

        if letter_sequence_is_xmas(letter1, letter2, letter3, letter4) {
            instances += 1;
        }
    }

    instances
}

fn letter_sequence_is_xmas(letter1: &String, letter2: &String, letter3: &String, letter4: &String) -> bool {
    letter1 == "X" && letter2 == "M" && letter3 == "A" && letter4 == "S"
}


fn solve_part02(file_path: &str) -> i64 {
    let lines: Vec<String> = parse_file_into_lines(file_path);

    let letter_rows = split_lines_into_rows_of_letters(lines);

    let mut instances: i64 = 0;

    for (i, letter_row) in letter_rows.iter().enumerate() {
        for (j, letter) in letter_row.iter().enumerate() {
            let has_space_to_right = j + 1 < letter_row.len();
            let has_space_to_bottom = i + 1 < letter_rows.len();
            let has_space_to_left = j >= 1;
            let has_space_to_top = i >= 1;

            if letter == "A" && has_space_to_right && has_space_to_bottom && has_space_to_left && has_space_to_top {
                let center_letter = letter;
                let top_left_letter = &letter_rows[i - 1][j - 1];
                let top_right_letter = &letter_rows[i - 1][j + 1];
                let bottom_left_letter = &letter_rows[i + 1][j - 1];
                let bottom_right_letter = &letter_rows[i + 1][j + 1];
                
                if letters_are_x_mas(center_letter, top_left_letter, top_right_letter, bottom_left_letter, bottom_right_letter) {
                    instances += 1;
                }
            }
        }
    }

    instances
}

fn letters_are_x_mas(
    center_letter: &String,
    top_left_letter: &String,
    top_right_letter: &String,
    bottom_left_letter: &String,
    bottom_right_letter: &String
) -> bool {
    let letter_combinations = [
        [top_left_letter, center_letter, bottom_right_letter],
        [bottom_right_letter, center_letter, top_left_letter],
        [bottom_left_letter, center_letter, top_right_letter],
        [top_right_letter, center_letter, bottom_left_letter]
    ];

    letter_combinations.iter().filter(|x| letters_are_mas(&x)).count() == 2
}

fn letters_are_mas(letters: &[&String; 3]) -> bool {
    letters[0] == "M" && letters[1] == "A" && letters[2] == "S"
}