use std::fs;

fn get_diff_vec(line_vec: Vec<i32>) -> Vec<i32> {
    let mut line_diff_vec = Vec::<i32>::new();

    if line_vec.len() > 1 {
        for n in 0..=(line_vec.len()-2) {
            line_diff_vec.push(line_vec[n+1]-line_vec[n]);
        }
    }

    return line_diff_vec;
}

fn is_safe(line_diff_vec: Vec<i32>) -> bool {
    if line_diff_vec.len() == 0 {
        return true;
    }

    let is_increasing = line_diff_vec[0] > 0;
    let mut is_safe = true;
    for &diff in line_diff_vec.iter() {
        if diff == 0 {
            is_safe = false;
            break;
        } else if is_increasing && diff < 0 {
            is_safe = false;
            break;
        } else if !is_increasing && diff > 0 {
            is_safe = false;
            break;
        } else if diff.abs() > 3 {
            is_safe = false;
            break;
        }
    }

    return is_safe;
}

fn main() {
    let input_file_contents = fs::read_to_string("src/input.txt").unwrap();

    let mut safe_reports_count = 0;
    let mut safe_reports_count_adj = 0;
    for line in input_file_contents.lines() {
        let line_vec: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let line_diff_vec = get_diff_vec(line_vec.to_vec());

        if is_safe(line_diff_vec) {
            safe_reports_count += 1;
            safe_reports_count_adj += 1;
        } else {
            for index_to_remove in 0..line_vec.len() {
                let mut line_vec_clone = line_vec.to_vec();
                line_vec_clone.remove(index_to_remove);
                if is_safe(get_diff_vec(line_vec_clone)) {
                    safe_reports_count_adj += 1;
                    break;
                }
            }
        }
    }

    println!("Safe reports count: {}", safe_reports_count);
    println!("Safe reports count (adjusted): {}", safe_reports_count_adj);
}
