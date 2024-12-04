use std::fs;

fn check_range_for_word(matrix: Vec<Vec<char>>, row_indexes: Vec<usize>, column_indexes: Vec<usize>) -> bool {
    let mut word = "".to_owned();
    for index in 0..4 {
        word.push(matrix[row_indexes[index]][column_indexes[index]]);
    }
    return word.eq("XMAS") || word.eq("SAMX");
}

fn main() {
    let input_string = fs::read_to_string("./src/input.txt").unwrap();
    let input_matrix = input_string.lines().map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();

    let mut xmas_count = 0;

    let row_size = input_matrix.len();
    for row in 0..row_size {
        let column_size = input_matrix[row].len();
        for column in 0..column_size {
            if row + 4 <= row_size && column + 4 <= column_size {
                if check_range_for_word(input_matrix.clone(), (row..(row+4)).collect(), (column..(column+4)).collect()) {
                    xmas_count += 1;
                }
            }
            if row + 4 <= row_size && column >= 3 {
                if check_range_for_word(input_matrix.clone(), (row..(row+4)).collect(), ((column-3)..(column+1)).rev().collect()) {
                    xmas_count += 1;
                }
            }
            if row + 4 <= row_size {
                if check_range_for_word(input_matrix.clone(), (row..(row+4)).collect(), vec![column; 4]) {
                    xmas_count += 1;
                }
            }
            if column + 4 <= column_size {
                if check_range_for_word(input_matrix.clone(), vec![row; 4], (column..(column+4)).collect()) {
                    xmas_count += 1;
                }
            }
        }
    }

    println!("Count is: {}", xmas_count);
}
