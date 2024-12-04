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
        "2024-day03" => (part_01_result, part_02_result) = solutions::year_2024::day03::solve(),
        "2024-day04" => (part_01_result, part_02_result) = solutions::year_2024::day04::solve(),
        "2024-day05" => (part_01_result, part_02_result) = solutions::year_2024::day05::solve(),
        "2024-day06" => (part_01_result, part_02_result) = solutions::year_2024::day06::solve(),
        "2024-day07" => (part_01_result, part_02_result) = solutions::year_2024::day07::solve(),
        "2024-day08" => (part_01_result, part_02_result) = solutions::year_2024::day08::solve(),
        "2024-day09" => (part_01_result, part_02_result) = solutions::year_2024::day09::solve(),
        "2024-day10" => (part_01_result, part_02_result) = solutions::year_2024::day10::solve(),
        "2024-day11" => (part_01_result, part_02_result) = solutions::year_2024::day11::solve(),
        "2024-day12" => (part_01_result, part_02_result) = solutions::year_2024::day12::solve(),
        "2024-day13" => (part_01_result, part_02_result) = solutions::year_2024::day13::solve(),
        "2024-day14" => (part_01_result, part_02_result) = solutions::year_2024::day14::solve(),
        "2024-day15" => (part_01_result, part_02_result) = solutions::year_2024::day15::solve(),
        "2024-day16" => (part_01_result, part_02_result) = solutions::year_2024::day16::solve(),
        "2024-day17" => (part_01_result, part_02_result) = solutions::year_2024::day17::solve(),
        "2024-day18" => (part_01_result, part_02_result) = solutions::year_2024::day18::solve(),
        "2024-day19" => (part_01_result, part_02_result) = solutions::year_2024::day19::solve(),
        "2024-day20" => (part_01_result, part_02_result) = solutions::year_2024::day20::solve(),
        "2024-day21" => (part_01_result, part_02_result) = solutions::year_2024::day21::solve(),
        "2024-day22" => (part_01_result, part_02_result) = solutions::year_2024::day22::solve(),
        "2024-day23" => (part_01_result, part_02_result) = solutions::year_2024::day23::solve(),
        "2024-day24" => (part_01_result, part_02_result) = solutions::year_2024::day24::solve(),
        "2024-day25" => (part_01_result, part_02_result) = solutions::year_2024::day25::solve(),
        _ => println!("No solution implemented for {solution_to_run} \n")
    }

    println!("Part 1 Result: {}\n", part_01_result);
    println!("Part 2 Result: {}\n", part_02_result);

    println!("Completed AoC solution run!");
}


