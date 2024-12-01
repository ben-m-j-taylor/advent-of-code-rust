mod util;
mod solutions;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let solution_to_run = &args[1];

    println!("Starting AoC solution run for {solution_to_run}...\n", );

    let mut part_01_result: i64 = 0;
    let mut part_02_result: i64 = 0;

    match solution_to_run.as_str() {
        "2024-day01" => (part_01_result, part_02_result) = solutions::year_2024::day01::solve(),
        "2024-day02" => (part_01_result, part_02_result) = solutions::year_2024::day02::solve(),
        _ => println!("No solution implemented for {solution_to_run} \n")
    }

    println!("Part 1 Result: {}\n", part_01_result);
    println!("Part 2 Result: {}\n", part_02_result);

    println!("Completed AoC solution run!");
}


