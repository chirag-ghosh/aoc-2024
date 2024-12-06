use std::{borrow::Borrow, collections::HashSet, fs};

#[derive(Clone)]
struct Point {
    is_obstacle: bool,
    is_visited: bool,
    visited_directions: HashSet<Direction>
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
enum Direction {
    UP, DOWN, RIGHT, LEFT
}

#[derive(Clone, Debug)]
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

fn process_matrix(input_matrix: Vec<Vec<Point>>, start_position: Position) -> (bool, HashSet<(usize, usize)>) {
    let mut matrix = input_matrix;
    let mut current_position = start_position;
    let mut path: HashSet<(usize, usize)> = HashSet::new();
    let mut is_loop = false;
    loop {
        if matrix[current_position.y][current_position.x].is_visited && matrix[current_position.y][current_position.x].visited_directions.contains(current_position.direction.borrow()) {
            is_loop = true;
            break;
        }
        matrix[current_position.y][current_position.x].is_visited = true;
        matrix[current_position.y][current_position.x].visited_directions.insert(current_position.direction);
        path.insert((current_position.y,current_position.x));

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

    (is_loop, path)
}

fn main() {
    let input_string = fs::read_to_string("./src/input.txt").unwrap();
    let mut matrix: Vec<Vec<Point>> = Vec::new();

    let mut start_position = Position {
        x: 0,
        y: 0,
        direction: Direction::UP
    };

    for (line_index, line) in input_string.lines().into_iter().enumerate() {
        let mut row: Vec<Point> = Vec::new();
        for (point_index, point) in line.chars().into_iter().enumerate() {
            row.push(Point { is_obstacle: point == '#', is_visited: false, visited_directions: HashSet::new() });
            if point == '^' {
                start_position = Position {
                    x: point_index,
                    y: line_index,
                    direction: Direction::UP
                }
            }
        }
        matrix.push(row);
    }

    let (_, path) = process_matrix(matrix.clone(), start_position.clone());
    println!("Visit count is: {}", path.len());

    let mut new_obstacles = 0;
    for point_index in path.iter() {
        matrix[point_index.0][point_index.1].is_obstacle = true;
        if process_matrix(matrix.clone(), start_position.clone()).0 {
            new_obstacles += 1;
        }
        matrix[point_index.0][point_index.1].is_obstacle = false;
    }
    println!("New obstacles: {}", new_obstacles-1);
}
