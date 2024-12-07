fn check_range_for_xmas(matrix: Vec<Vec<char>>, row_indexes: Vec<usize>, column_indexes: Vec<usize>) -> bool {
    let mut word = "".to_owned();
    for index in 0..4 {
        word.push(matrix[row_indexes[index]][column_indexes[index]]);
    }
    return word.eq("XMAS") || word.eq("SAMX");
}

fn check_range_for_mas(matrix: Vec<Vec<char>>, row_indexes: Vec<usize>, column_indexes: Vec<usize>) -> bool {
    let mut word = "".to_owned();
    for index in 0..3 {
        word.push(matrix[row_indexes[index]][column_indexes[index]]);
    }
    return word.eq("MAS") || word.eq("SAM");
}

pub fn main(input_string: String) -> (String, String) {
    let input_matrix = input_string.lines().map(|line| line.chars().collect()).collect::<Vec<Vec<char>>>();

    let mut xmas_count = 0;

    let row_size = input_matrix.len();
    for row in 0..row_size {
        let column_size = input_matrix[row].len();
        for column in 0..column_size {
            if row + 4 <= row_size && column + 4 <= column_size {
                if check_range_for_xmas(input_matrix.clone(), (row..(row+4)).collect(), (column..(column+4)).collect()) {
                    xmas_count += 1;
                }
            }
            if row + 4 <= row_size && column >= 3 {
                if check_range_for_xmas(input_matrix.clone(), (row..(row+4)).collect(), ((column-3)..(column+1)).rev().collect()) {
                    xmas_count += 1;
                }
            }
            if row + 4 <= row_size {
                if check_range_for_xmas(input_matrix.clone(), (row..(row+4)).collect(), vec![column; 4]) {
                    xmas_count += 1;
                }
            }
            if column + 4 <= column_size {
                if check_range_for_xmas(input_matrix.clone(), vec![row; 4], (column..(column+4)).collect()) {
                    xmas_count += 1;
                }
            }
        }
    }

    let mut cross_xmass_count = 0;

    let row_size = input_matrix.len();
    for row in 1..(row_size - 1) {
        let column_size = input_matrix[row].len();
        for column in 1..(column_size - 1) {
            if input_matrix[row][column] == 'A' {
                if check_range_for_mas(input_matrix.clone(), ((row-1)..=(row+1)).collect(), ((column-1)..=(column+1)).collect())
                && check_range_for_mas(input_matrix.clone(), ((row-1)..=(row+1)).collect(), ((column-1)..=(column+1)).rev().collect()) {
                    cross_xmass_count += 1;
                }
            }
        }
    }

    (xmas_count.to_string(), cross_xmass_count.to_string())
}
