use crate::util::parse_file_into_lines;

pub fn solve() -> (i64, i64) {
    const FILE_PATH: &str = "./input_data/2024/day06/full-input.txt";

    (
        solve_part01(FILE_PATH),
        solve_part02(),
    )
}

struct Vec2 {
    x: i64,
    y: i64,
}

impl Vec2 {
    fn new(x: i64, y: i64) -> Self {
        Vec2 { x, y }
    }
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn solve_part01(file_path: &str) -> i64 {
    let lines = parse_file_into_lines(file_path);

    let mut rows = split_lines(lines);

    let mut position: Vec2;
    let mut direction: Dir;

    (position, direction) = find_start_position_and_direction(&rows);
    
    let mut positions_visited: Vec<Vec2> = Vec::new();
    
    positions_visited.push(Vec2::new(position.x, position.y));
    
    loop {

        // println!("position before move - x = {}, y = {}", position.x, position.y);

        // println!("{:?}", rows);

        match direction {
            Dir::Up => {
                let next_position = Vec2::new(position.x, position.y - 1);

                if !position_within_bounds(&next_position, &rows) {
                    break;
                }

                let next_character: &String = &rows[next_position.y as usize][next_position.x as usize];

                // println!("next_character = {}", next_character);

                if next_character == "#" {
                    direction = Dir::Right;
                } else {
                    rows[next_position.y as usize][next_position.x as usize] = "X".to_string();
                    
                    if !positions_visited.iter().any(|v| v.x == next_position.x && v.y == next_position.y) {
                        positions_visited.push(Vec2::new(next_position.x, next_position.y));
                    }
                    
                    position = next_position;
                }
            },
            Dir::Down => {
                let next_position = Vec2::new(position.x, position.y + 1);

                if !position_within_bounds(&next_position, &rows) {
                    break;
                }

                let next_character: &String = &rows[next_position.y as usize][next_position.x as usize];

                // println!("next_character = {}", next_character);

                if next_character == "#" {
                    direction = Dir::Left;
                } else {
                    rows[next_position.y as usize][next_position.x as usize] = "X".to_string();

                    if !positions_visited.iter().any(|v| v.x == next_position.x && v.y == next_position.y) {
                        positions_visited.push(Vec2::new(next_position.x, next_position.y));
                    }
                    
                    position = next_position;
                }
            },
            Dir::Left => {
                let next_position = Vec2::new(position.x - 1, position.y);

                if !position_within_bounds(&next_position, &rows) {
                    break;
                }

                let next_character: &String = &rows[next_position.y as usize][next_position.x as usize];

                // println!("next_character = {}", next_character);

                if next_character == "#" {
                    direction = Dir::Up;
                } else {
                    rows[next_position.y as usize][next_position.x as usize] = "X".to_string();

                    if !positions_visited.iter().any(|v| v.x == next_position.x && v.y == next_position.y) {
                        positions_visited.push(Vec2::new(next_position.x, next_position.y));
                    }

                    position = next_position;
                }
            },
            Dir::Right => {
                let next_position = Vec2::new(position.x + 1, position.y);

                if !position_within_bounds(&next_position, &rows) {
                    break;
                }

                let next_character: &String = &rows[next_position.y as usize][next_position.x as usize];

                // println!("next_character = {}", next_character);

                if next_character == "#" {
                    direction = Dir::Down;
                } else {
                    rows[next_position.y as usize][next_position.x as usize] = "X".to_string();

                    if !positions_visited.iter().any(|v| v.x == next_position.x && v.y == next_position.y) {
                        positions_visited.push(Vec2::new(next_position.x, next_position.y));
                    }

                    position = next_position;
                }
            }
        }

        // println!("position after move - x = {}, y = {}", position.x, position.y);
    }

    positions_visited.len() as i64 
}

fn split_lines(lines: Vec<String>) -> Vec<Vec<String>> {
    lines
        .iter()
        .map(|l| l
            .chars()
            .map(String::from)
            .collect()
        )
        .collect()
}

fn find_start_position_and_direction(rows: &Vec<Vec<String>>) -> (Vec2, Dir) {
    for (i, row) in rows.iter().enumerate() {
        for (j, character) in row.iter().enumerate() {
            let position = Vec2::new(j as i64, i as i64);

            match character.as_str() {
                "^" => return (position, Dir::Up),
                ">" => return (position, Dir::Right),
                "v" => return (position, Dir::Down),
                "<" => return (position, Dir::Left),
                _ => continue,
            }
        }
    }

    panic!("No start position found");
}

fn position_within_bounds(position: &Vec2, rows: &Vec<Vec<String>>) -> bool {
    let bounds = Vec2::new(rows[0].len() as i64, rows.len() as i64);

    position.x >= 0 && position.y >= 0 && position.x < bounds.x && position.y < bounds.y
}

fn solve_part02() -> i64 {
    0
}