use std::fs;
use regex::Regex;

fn get_sum(input_string: &str) -> i32 {
    let re = Regex::new(r"mul\((?<first>[0-9]{1,3}),(?<second>[0-9]{1,3})\)").unwrap();
    let instructions: Vec<(i32,i32)> = re.captures_iter(input_string).map(|m| {
        let first_operand = m.name("first").unwrap().as_str().parse::<i32>().unwrap();
        let second_operand = m.name("second").unwrap().as_str().parse::<i32>().unwrap();
        (first_operand,second_operand)
    }).collect();
    
    let mut sum = 0;
    for (first_operand, second_operand) in instructions.iter() {
        sum += first_operand * second_operand;
    }

    sum
}

fn main() {
    let input_string = fs::read_to_string("./src/input.txt").unwrap();
    println!("Normal sum is: {}", get_sum(&input_string));

    let chunks: Vec<&str> = input_string.split("do()").collect();
    let mut conditional_input_string: String = "".to_owned();
    for chunk in chunks.iter() {
        if chunk.contains("don't()") {
            conditional_input_string.push_str(chunk.split("don't()").collect::<Vec<&str>>()[0]);
        } else {
            conditional_input_string.push_str(chunk);
        }
    }
    println!("Conditional sum is: {}", get_sum(&conditional_input_string));
}
