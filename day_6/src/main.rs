use std::fs;

struct Point {
    is_obstacle: bool,
    is_visited: bool
}

#[derive(PartialEq, Eq)]
enum Direction {
    UP, DOWN, RIGHT, LEFT
}

struct Position {
    x: usize,
    y: usize,
    direction: Direction
}

impl Position {
    fn turn(&mut self) {
        match self.direction {
            Direction::UP => self.direction = Direction::RIGHT,
            Direction::RIGHT => self.direction = Direction::DOWN,
            Direction::DOWN => self.direction = Direction::LEFT,
            Direction::LEFT => self.direction = Direction::UP,
        }
    }

    fn get_next_position_index(&self) -> (usize,usize) {
        match self.direction {
            Direction::UP => return (self.x, self.y-1),
            Direction::RIGHT => return (self.x+1, self.y),
            Direction::DOWN => return (self.x, self.y+1),
            Direction::LEFT => return (self.x-1, self.y),
        }
    }

    fn walk(&mut self) {
        match self.direction {
            Direction::UP => self.y -= 1,
            Direction::RIGHT => self.x += 1,
            Direction::DOWN => self.y += 1,
            Direction::LEFT => self.x -= 1,
        }
    }
}

fn main() {
    let input_string = fs::read_to_string("./src/input.txt").unwrap();
    let mut matrix: Vec<Vec<Point>> = Vec::new();

    let mut current_position = Position {
        x: 0,
        y: 0,
        direction: Direction::UP
    };

    for (line_index, line) in input_string.lines().into_iter().enumerate() {
        let mut row: Vec<Point> = Vec::new();
        for (point_index, point) in line.chars().into_iter().enumerate() {
            row.push(Point { is_obstacle: point == '#', is_visited: false });
            if point == '^' {
                current_position = Position {
                    x: point_index,
                    y: line_index,
                    direction: Direction::UP
                }
            }
        }
        matrix.push(row);
    }

    loop {
        matrix[current_position.y][current_position.x].is_visited = true;

        if current_position.direction == Direction::UP && current_position.y == 0
        || current_position.direction == Direction::RIGHT && current_position.x == matrix[0].len()-1
        || current_position.direction == Direction::DOWN && current_position.y == matrix.len()-1
        || current_position.direction == Direction::LEFT && current_position.x == 0 {
            break;
        }

        let (next_x,next_y) = current_position.get_next_position_index();
        if matrix[next_y][next_x].is_obstacle {
            current_position.turn();
        } else {
            current_position.walk();
        }
    }

    let mut visit_count = 0;
    for row in matrix.iter() {
        for point in row.iter() {
            if point.is_visited {
                visit_count += 1;
            }
        }
    }

    println!("Visit count is: {}", visit_count);
}
