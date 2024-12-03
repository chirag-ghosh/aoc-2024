use std::fs;
use regex::Regex;

fn main() {
    let input_string = fs::read_to_string("./src/input.txt").unwrap();
    let re = Regex::new(r"mul\((?<first>[0-9]{1,3}),(?<second>[0-9]{1,3})\)").unwrap();
    let instructions: Vec<(i32,i32)> = re.captures_iter(&input_string).map(|m| {
        let first_operand = m.name("first").unwrap().as_str().parse::<i32>().unwrap();
        let second_operand = m.name("second").unwrap().as_str().parse::<i32>().unwrap();
        (first_operand,second_operand)
    }).collect();
    
    let mut sum = 0;
    for (first_operand, second_operand) in instructions.iter() {
        sum += first_operand * second_operand;
    }
    println!("Sum is: {}", sum);
}
