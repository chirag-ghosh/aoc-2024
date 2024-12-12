const INCREMENTS: [(i32, i32); 4] = [(-1,0), (1,0), (0,-1), (0,1)];

fn traverse_region(field: &mut Vec<Vec<(char, bool)>>, index: (usize, usize)) -> (i32, i32) {
    let mut area = 0;
    let mut perimeter = 0;

    if !field[index.0][index.1].1 {
        area += 1;
        field[index.0][index.1].1 = true;

        for increment in INCREMENTS.into_iter() {
            let row_index = index.0 as i32 + increment.0;
            let column_index = index.1 as i32 + increment.1;

            if column_index >= 0 && column_index < field[0].len() as i32 && row_index >= 0 && row_index < field.len() as i32 {
                if field[row_index as usize][column_index as usize].0 == field[index.0][index.1].0 {
                    let (side_area, side_perimeter) = traverse_region(field, (row_index as usize, column_index as usize));
                    area += side_area;
                    perimeter += side_perimeter;
                } else {
                    perimeter += 1;
                }
            } else {
                perimeter += 1;
            }
        }
    }

    (area, perimeter)
}

pub fn main(input_string: String) -> (String, String) {
    let mut field: Vec<Vec<(char, bool)>> = Vec::new();

    for line in input_string.lines().into_iter() {
        let mut line_vec: Vec<(char, bool)> = Vec::new();
        for plot in line.chars().into_iter() {
            line_vec.push((plot, false));
        }
        field.push(line_vec);
    }

    let mut total_price = 0;
    for row_index in 0..field.len() {
        for column_index in 0..field[0].len() {

            if !field[row_index][column_index].1 {
                let (area, perimeter) = traverse_region(&mut field, (row_index, column_index));
                total_price += area * perimeter;
            }
        }
    }
    (total_price.to_string(), 0.to_string())
}
