use std::collections::HashMap;

const INCREMENTS: [(i32, i32); 4] = [(-1,0), (1,0), (0,-1), (0,1)];

fn traverse_map(map: Vec<Vec<u32>>, index: (i32, i32), peaks: &mut HashMap<(usize, usize), bool>) {
    if index.0 >= map.len() as i32 || index.0 < 0 || index.1 >= map[0].len() as i32 || index.1 < 0 {
        return;
    }

    if map[index.0 as usize][index.1 as usize] == 9 {
        peaks.insert((index.0 as usize, index.1 as usize), true);
    }

    for increment in INCREMENTS.into_iter() {
        let next_index = (index.0+increment.0, index.1+increment.1);
        if next_index.0 >= map.len() as i32 || next_index.0 < 0 || next_index.1 >= map[0].len() as i32 || next_index.1 < 0 {
            continue;
        }

        if map[next_index.0 as usize][next_index.1 as usize] == map[index.0 as usize][index.1 as usize] + 1 {
            traverse_map(map.to_owned(), next_index, peaks);
        }
    }
}

pub fn main(input_string: String) -> (String, String) {
    let mut map: Vec<Vec<u32>> = Vec::new();
    let mut peaks: HashMap<(usize, usize), bool> = HashMap::new();
    let mut trailheads: Vec<(usize, usize)> = Vec::new();
    for (line_index, line) in input_string.lines().into_iter().enumerate() {
        let mut line_vec: Vec<u32> = Vec::new();
        for (point_index, point) in line.chars().enumerate() {
            line_vec.push(point as u32 - '0' as u32);
            if point == '9' {
                peaks.insert((line_index, point_index), false);
            } else if point == '0' {
                trailheads.push((line_index, point_index));
            }
        }
        map.push(line_vec);
    }
    
    let mut sum = 0;
    for trailhead in trailheads.into_iter() {
        let mut peaks_clone = peaks.clone();
        traverse_map(map.clone(), (trailhead.0 as i32, trailhead.1 as i32), &mut peaks_clone);
        let mut peaks_conquered = 0;
        peaks_clone.into_values().for_each(|value| {
            if value {
                peaks_conquered += 1;
            }
        });
        
        sum += peaks_conquered;
    }

    (sum.to_string(), 0.to_string())
}
