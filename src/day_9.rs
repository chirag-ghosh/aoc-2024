fn get_checksum(disk: &Vec<i32>) -> i64 {
    let mut checksum = 0;
    for (index, digit) in disk.clone().into_iter().enumerate() {
        if digit == -1 {
            continue;
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
    let mut files: Vec<(usize, usize)> = Vec::new();
    let mut free_spaces: Vec<(usize, usize)> = Vec::new();
    for (index, digit) in input_string.trim().chars().map(|c| c as u32 - '0' as u32).enumerate() {
        if index % 2 == 0 {
            files.push((disk.len(), digit as usize));
            disk.append(&mut vec![(index/2) as i32; digit as usize]);
        } else {
            free_spaces.push((disk.len(), digit as usize));
            disk.append(&mut vec![-1; digit as usize]);
        }
    }

    let mut disk_for_block_frag = disk.clone(); 
    let mut current_index = disk.len() - 1;
    loop {
        if disk_for_block_frag[current_index] == -1 {
            current_index -= 1;
            continue;
        }

        let first_index_of_free_space = get_first_index_of_free_space(&disk_for_block_frag); // This gives byte-index but works in our case since we are dealing with ascii only
        if first_index_of_free_space > current_index {
            break;
        } else {
            disk_for_block_frag.swap(current_index, first_index_of_free_space);
        }
        current_index -= 1;
    }
    
    let mut disk_for_file_frag = disk.clone();
    for (file_index, file_size) in files.iter_mut().rev() {
        for (free_space_index, free_space_size) in free_spaces.iter_mut() {
            if file_index.clone() > free_space_index.clone() && file_size.clone() <= free_space_size.clone() {
                for i in 0..(file_size.clone() as usize) {
                    disk_for_file_frag.swap(free_space_index.clone() + i, file_index.clone() + i);
                }
                *free_space_index = free_space_index.clone() + file_size.clone();
                *free_space_size = free_space_size.clone() - file_size.clone();
                break;
            }
        }
    }

    (get_checksum(&disk_for_block_frag).to_string(), get_checksum(&disk_for_file_frag).to_string())
}