use std::collections::HashMap;

fn get_stone_line_len(lookup: &mut HashMap<(String, i32), i64>, input_stone: String, remaining_blinks: i32) -> i64 {
    if remaining_blinks == 0 {
        return 1;
    }

    if lookup.contains_key(&(input_stone.to_owned(), remaining_blinks)) {
        return *lookup.get(&(input_stone, remaining_blinks)).unwrap();
    }

    let mut num_stones = 0;
    if input_stone == "0" {
        num_stones = get_stone_line_len(lookup, "1".to_owned(), remaining_blinks - 1);
    } else if input_stone.len() % 2 == 0 {
        let stone_split = input_stone.split_at(input_stone.len() / 2);
        num_stones += get_stone_line_len(lookup, stone_split.0.parse::<i64>().unwrap().to_string(), remaining_blinks - 1);
        num_stones += get_stone_line_len(lookup, stone_split.1.parse::<i64>().unwrap().to_string(), remaining_blinks - 1);
    } else {
        num_stones = get_stone_line_len(lookup, (input_stone.parse::<i64>().unwrap() * 2024).to_string(), remaining_blinks - 1)
    }

    lookup.insert((input_stone, remaining_blinks), num_stones);

    return num_stones;
}

pub fn main(input_string: String) -> (String, String) {
    let stone_line = input_string.trim().split_whitespace().map(String::from).collect::<Vec<String>>();

    let mut lookup: HashMap<(String, i32), i64> = HashMap::new();

    let mut stone_len_25 = 0;
    for stone in stone_line.to_owned().into_iter() {
        stone_len_25 += get_stone_line_len(&mut lookup, stone, 25);
    }

    let mut stone_len_75 = 0;
    for stone in stone_line.to_owned().into_iter() {
        stone_len_75 += get_stone_line_len(&mut lookup, stone, 75);
    }

    (stone_len_25.to_string(), stone_len_75.to_string())
}
