use std::collections::{HashMap, HashSet};

fn check_point_bounds(point: (i32,i32), rows_len: i32, columns_len: i32) -> bool {
    return (0..columns_len).contains(&(point.0)) && (0..rows_len).contains(&(point.1));
} 

pub fn main(input_string: String) -> (String, String) {
    let lines = input_string.lines().collect::<Vec<&str>>();
    let rows_len = lines.len() as i32;
    let columns_len = lines[0].len() as i32;

    let mut antenna_map: HashMap<char, Vec<(i32,i32)>> = HashMap::new();
    for (line_index, line) in lines.into_iter().enumerate() {
        for (point_index, point) in line.chars().into_iter().enumerate() {
            if point == '.' {
                continue;
            }
            if antenna_map.contains_key(&point) {
                antenna_map.get_mut(&point).unwrap().push((point_index as i32, line_index as i32));
            } else {
                antenna_map.insert(point, vec![(point_index as i32, line_index as i32)]);
            }
        }
    }

    let mut antinodes: HashSet<(i32,i32)> = HashSet::new();
    for (_, antenna_locations) in antenna_map.into_iter() {
        for i in 0..(antenna_locations.len()-1) {
            for j in (i+1)..antenna_locations.len() {
                let new_point_1 = (
                    antenna_locations[i].0 + (antenna_locations[i].0 - antenna_locations[j].0),
                    antenna_locations[i].1 + (antenna_locations[i].1 - antenna_locations[j].1));
                if check_point_bounds(new_point_1, rows_len, columns_len) {
                    antinodes.insert(new_point_1);
                }

                let new_point_2 = (
                    antenna_locations[j].0 + (antenna_locations[j].0 - antenna_locations[i].0),
                    antenna_locations[j].1 + (antenna_locations[j].1 - antenna_locations[i].1));
                if check_point_bounds(new_point_2, rows_len, columns_len) {
                    antinodes.insert(new_point_2);
                }
            }
        }
    }

    (antinodes.len().to_string(), 0.to_string())
}