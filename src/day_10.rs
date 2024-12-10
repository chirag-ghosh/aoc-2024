use std::collections::HashMap;

const INCREMENTS: [(i32, i32); 4] = [(-1,0), (1,0), (0,-1), (0,1)];

fn traverse_map(map: Vec<Vec<u32>>, index: (usize, usize), peaks: &mut HashMap<(usize, usize), i32>) {
    if index.0 >= map.len() || index.1 >= map[0].len() {
        return;
    }

    if map[index.0][index.1] == 9 {
        peaks.insert(index, peaks.get(&index).unwrap() + 1);
    }

    for increment in INCREMENTS.into_iter() {
        let next_line_index = index.0 as i32 +increment.0;
        let next_point_index = index.1 as i32 +increment.1;
        if next_line_index >= map.len() as i32 || next_line_index < 0 || next_point_index >= map[0].len() as i32 || next_point_index < 0 {
            continue;
        }
        let next_index = (next_line_index as usize, next_point_index as usize);
        if map[next_index.0][next_index.1] == map[index.0][index.1] + 1 {
            traverse_map(map.to_owned(), next_index, peaks);
        }
    }
}

pub fn main(input_string: String) -> (String, String) {
    let mut map: Vec<Vec<u32>> = Vec::new();
    let mut peaks: HashMap<(usize, usize), i32> = HashMap::new();
    let mut trailheads: Vec<(usize, usize)> = Vec::new();
    for (line_index, line) in input_string.lines().into_iter().enumerate() {
        let mut line_vec: Vec<u32> = Vec::new();
        for (point_index, point) in line.chars().enumerate() {
            line_vec.push(point as u32 - '0' as u32);
            if point == '9' {
                peaks.insert((line_index, point_index), 0);
            } else if point == '0' {
                trailheads.push((line_index, point_index));
            }
        }
        map.push(line_vec);
    }
    
    let mut sum_of_trailheads = 0;
    let mut sum_of_ratings = 0;
    for trailhead in trailheads.into_iter() {
        let mut peaks_clone = peaks.clone();
        traverse_map(map.clone(), (trailhead.0, trailhead.1), &mut peaks_clone);

        let mut peaks_conquered = 0;
        let mut rating = 0;
        peaks_clone.into_values().for_each(|value| {
            if value >= 1 {
                peaks_conquered += 1;
                rating += value;
            }
        });
        
        sum_of_trailheads += peaks_conquered;
        sum_of_ratings += rating;
    }

    (sum_of_trailheads.to_string(), sum_of_ratings.to_string())
}
