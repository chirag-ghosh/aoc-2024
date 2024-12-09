fn get_checksum(disk: &Vec<i32>) -> i64 {
    let mut checksum = 0;
    for (index, digit) in disk.clone().into_iter().enumerate() {
        if digit == -1 {
            break;
        }
        checksum += index as i64 * digit as i64;
    }

    checksum
}

fn get_first_index_of_free_space(disk: &Vec<i32>) -> usize {
    for (index, digit) in disk.clone().into_iter().enumerate() {
        if digit == -1 {
            return index;
        }
    }
    return disk.len();
}

pub fn main(input_string: String) -> (String, String) {
    let mut disk: Vec<i32> = Vec::new();
    for (index, digit) in input_string.trim().chars().map(|c| c as u32 - '0' as u32).enumerate() {
        if index % 2 == 0 {
            disk.append(&mut vec![(index/2) as i32; digit as usize]);
        } else {
            disk.append(&mut vec![-1; digit as usize]);
        }
    }
    
    let mut current_index = disk.len() - 1;
    loop {
        if disk[current_index] == -1 {
            current_index -= 1;
            continue;
        }

        let first_index_of_free_space = get_first_index_of_free_space(&disk); // This gives byte-index but works in our case since we are dealing with ascii only
        if first_index_of_free_space > current_index {
            break;
        } else {
            disk.swap(current_index, first_index_of_free_space);
        }
        current_index -= 1;
    }

    (get_checksum(&disk).to_string(), 0.to_string())
}