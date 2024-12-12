pub fn main(input_string: String) -> (String, String) {
    let mut stone_line = input_string.trim().split_whitespace().map(String::from).collect::<Vec<String>>();

    for _ in 0..25 {
        let mut stone_line_new: Vec<String> = Vec::new();
        for stone in stone_line.to_owned().into_iter() {
            if stone == "0" {
                stone_line_new.push("1".to_owned());
            } else if stone.len() % 2 == 0 {
                let stone_split = stone.split_at(stone.len() / 2);
                stone_line_new.push(stone_split.0.parse::<i64>().unwrap().to_string());
                stone_line_new.push(stone_split.1.parse::<i64>().unwrap().to_string());
            } else {
                let stone_value = stone.parse::<i64>().unwrap();
                stone_line_new.push((stone_value * 2024).to_string());
            }
        }
        stone_line = stone_line_new;
    }

    (stone_line.len().to_string(), 0.to_string())
}
