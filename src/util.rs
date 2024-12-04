use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs;

pub fn read_file_to_string<P>(filename: P) -> String
where
    P: AsRef<Path>,
{
    fs::read_to_string(filename).expect("Unable to read file")
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn parse_file_into_lines(file_path: &str) -> Vec<String> {
    let lines = read_lines(file_path);

    match lines {
        Ok(lines) => {
            let mut ok_lines: Vec<String> = Vec::new();

            for line in lines {
                match line {
                    Ok(line) => {
                        ok_lines.push(line);
                    },
                    Err(e) => {
                        panic!("{}", e);
                    }
                }
            }

            ok_lines
        }
        _ => {
            panic!("Failed to read input data");
        }
    }
}
