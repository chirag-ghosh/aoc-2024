use std::collections::HashMap;

pub fn main(input_string: String) -> (String,String) {
    let mut first_vector = Vec::<i32>::new();
    let mut second_vector = Vec::<i32>::new();
    let mut first_map = HashMap::<i32, i32>::new();
    let mut second_map = HashMap::<i32, i32>::new();

    for line in input_string.lines() {
        let line_split = line.split_whitespace().collect::<Vec<&str>>();
        let first_entry = line_split[0].parse::<i32>().unwrap();
        let second_entry = line_split[1].parse::<i32>().unwrap();

        first_vector.push(first_entry);
        first_map.entry(first_entry).and_modify(|e| *e += 1).or_insert(1);

        second_vector.push(second_entry);
        second_map.entry(second_entry).and_modify(|e| *e += 1).or_insert(1);
    }

    first_vector.sort();
    second_vector.sort();

    let mut distance = 0;
    for i in 0..first_vector.len() {
        distance += (first_vector[i] - second_vector[i]).abs();
    }

    let mut similarity = 0;
    for key in first_map.keys() {
        if second_map.contains_key(key) {
            similarity += key * second_map.get(key).unwrap();
        } 
    }

    (distance.to_string(), similarity.to_string())
}