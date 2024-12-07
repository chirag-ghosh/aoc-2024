use std::borrow::Borrow;

fn process_operands(operands: Vec<i64>, index: usize, value: i64, result: i64, is_concatenation_allowed: bool) -> bool {
    if index == operands.len() {
        return value == result;
    }

    if value > result {
        return false;
    }

    let new_prod_value = value * operands[index];
    if process_operands(operands.to_owned(), index + 1, new_prod_value, result, is_concatenation_allowed) {
        return true;
    }
    
    let new_sum_value = value + operands[index];
    if process_operands(operands.to_owned(), index + 1, new_sum_value, result, is_concatenation_allowed) {
        return true;
    }

    if is_concatenation_allowed {
        let new_concat_value = (value.to_string() + operands[index].to_string().borrow()).parse::<i64>().unwrap();
        if process_operands(operands, index + 1, new_concat_value, result, is_concatenation_allowed) {
            return true;
        }
    }

    false
}

pub fn main(input_string: String) -> (String, String) {
    let mut sum = 0;
    let mut concat_sum = 0;
    for line in input_string.lines().into_iter() {
        let line_split = line.split(":").collect::<Vec<&str>>();
        let result = line_split[0].parse::<i64>().unwrap();
        let operands = line_split[1].trim().split_whitespace().map(|value| value.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        if process_operands(operands.to_owned(), 1, operands[0], result, false) {
            sum += result;
        }
        if process_operands(operands.to_owned(), 1, operands[0], result, true) {
            concat_sum += result;
        }
    }

    (sum.to_string(), concat_sum.to_string())
}
