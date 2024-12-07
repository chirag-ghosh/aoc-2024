use std::fs;

fn process_operands(operands: Vec<i64>, index: usize, value: i64, result: i64) -> bool {
    if index == operands.len() {
        return value == result;
    }

    if value > result {
        return false;
    }

    let new_prod_value = value * operands[index];
    let new_sum_value = value + operands[index];

    return process_operands(operands.to_owned(), index + 1, new_prod_value, result) || process_operands(operands.to_owned(), index + 1, new_sum_value, result)
}

fn main() {
    let input_string = fs::read_to_string("./src/input.txt").unwrap();

    let mut sum = 0;
    for line in input_string.lines().into_iter() {
        let line_split = line.split(":").collect::<Vec<&str>>();
        let result = line_split[0].parse::<i64>().unwrap();
        let operands = line_split[1].trim().split_whitespace().map(|value| value.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        if process_operands(operands.to_owned(), 1, operands[0], result) {
            sum += result;
        }
    }
    println!("Sum is: {}", sum);
}
