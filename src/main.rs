mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;

use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Please provide the day number as cmd-line argument");
        return;
    }

    let day = args[1].parse::<i32>().unwrap();

    let input_dir_path;
    if args.len() >2 && args[2].eq_ignore_ascii_case("--sample") {
        input_dir_path = "./src/sample_input";
    } else {
        input_dir_path = "./src/input";
    }

    let input_string = fs::read_to_string(format!("{input_dir_path}/{day}.txt")).unwrap();

    let (part1_ans, part2_ans) = match day {
        1 => day_1::main(input_string),
        2 => day_2::main(input_string),
        3 => day_3::main(input_string),
        4 => day_4::main(input_string),
        5 => day_5::main(input_string),
        6 => day_6::main(input_string),
        7 => day_7::main(input_string),
        _ => {
            println!("Invalid day");
            ("".to_owned(), "".to_owned())
        }
    };

    println!("Answer to part 1 is: {}", part1_ans);
    println!("Answer to part 2 is: {}", part2_ans);
}
