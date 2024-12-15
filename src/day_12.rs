const INCREMENTS: [(i32, i32); 4] = [(-1,0), (1,0), (0,-1), (0,1)];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    RIGHT,
    DOWN,
    LEFT,
    UP
}

impl Direction {
    fn are_opposites(self, another_direction: Direction) -> bool {
        match self {
            Direction::RIGHT => return another_direction == Direction::LEFT,
            Direction::DOWN => return another_direction == Direction::UP,
            Direction::LEFT => return another_direction == Direction::RIGHT,
            Direction::UP => return another_direction == Direction::DOWN,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    start: (usize, usize),
    end: (usize, usize),
    direction: Direction
}

impl Line {
    fn is_equal(self, another_line: Line) -> bool {
        if self.start.0 == another_line.start.0 && self.start.1 == another_line.start.1 && self.end.0 == another_line.end.0 && self.end.1 == another_line.end.1 && self.direction == another_line.direction {
            return true;
        }
        if self.start.0 == another_line.end.0 && self.start.1 == another_line.end.1 && self.end.0 == another_line.start.0 && self.end.1 == another_line.start.1 && self.direction.are_opposites(another_line.direction) {
            return true;
        }
        return false;
    }

    fn is_prev(self, another_line: Line) -> bool {
        if self.end.0 == another_line.start.0 && self.end.1 == another_line.start.1 {
            return true;
        }
        return false;
    }
}

// This ensures that only sides which are on the perimeter are added
fn add_or_remove_if_present(lines: &mut Vec<Line>, line: Line) {
    if lines.iter().any(|existing_line| existing_line.is_equal(line)) {
        lines.retain(|&existing_line| !existing_line.is_equal(line));
    } else {
        lines.push(line);
    }
}

fn add_plot_lines(lines: &mut Vec<Line>, plot_index: (usize, usize)) {
    add_or_remove_if_present(lines, Line { start: (plot_index.0, plot_index.1), end: (plot_index.0, plot_index.1 + 1), direction: Direction::RIGHT });
    add_or_remove_if_present(lines, Line { start: (plot_index.0, plot_index.1 + 1), end: (plot_index.0 + 1, plot_index.1 + 1), direction: Direction::DOWN });
    add_or_remove_if_present(lines, Line { start: (plot_index.0 + 1, plot_index.1 + 1), end: (plot_index.0 + 1, plot_index.1), direction: Direction::LEFT });
    add_or_remove_if_present(lines, Line { start: (plot_index.0 + 1, plot_index.1), end: (plot_index.0, plot_index.1), direction: Direction::UP });
}

fn sort_lines(lines: &mut Vec<Line>) {
    for i in 0..(lines.len() - 1) {
        for j in (i+1)..lines.len() {
            if lines[i].is_prev(lines[j]) && i+1 != j {
                let temp = lines[i+1];
                lines[i+1] = lines[j];
                lines[j] = temp;
            }
        }
    }
}

fn count_sides(lines: Vec<Line>) -> i32 {
    let mut sides = 1;

    let mut first_index_of_contiguous_side = 0;
    for line_index in 0..(lines.len()-1) {
        if lines[line_index].is_prev(lines[line_index+1]) {
            if lines[line_index].direction != lines[line_index+1].direction {
                sides += 1;
            }
        } else {
            sides += 1;
        }

        if !lines[line_index].is_prev(lines[line_index+1]) {
            if lines[first_index_of_contiguous_side].direction == lines[line_index].direction {
                sides -= 1;
            }
            first_index_of_contiguous_side = line_index + 1;
        } else if line_index == lines.len()-2 {
            if lines[first_index_of_contiguous_side].direction == lines[line_index+1].direction {
                sides -= 1;
            }
        }
    }
    return sides;
}

fn traverse_region(field: &mut Vec<Vec<(char, bool)>>, index: (usize, usize), region_lines: &mut Vec<Line>) -> (i32, i32) {
    let mut area = 0;
    let mut perimeter = 0;

    if !field[index.0][index.1].1 {
        area += 1;
        field[index.0][index.1].1 = true;
        add_plot_lines(region_lines, index);

        for increment in INCREMENTS.into_iter() {
            let row_index = index.0 as i32 + increment.0;
            let column_index = index.1 as i32 + increment.1;

            if column_index >= 0 && column_index < field[0].len() as i32 && row_index >= 0 && row_index < field.len() as i32 {
                if field[row_index as usize][column_index as usize].0 == field[index.0][index.1].0 {
                    let (side_area, side_perimeter) = traverse_region(field, (row_index as usize, column_index as usize), region_lines);
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
    let mut total_discounted_price = 0;
    for row_index in 0..field.len() {
        for column_index in 0..field[0].len() {

            if !field[row_index][column_index].1 {
                let mut region_lines: Vec<Line> = Vec::new();

                let (area, perimeter) = traverse_region(&mut field, (row_index, column_index), &mut region_lines);
                total_price += area * perimeter;

                sort_lines(&mut region_lines);
                total_discounted_price += area * count_sides(region_lines.clone());
            }
        }
    }
    (total_price.to_string(), total_discounted_price.to_string())
}
