use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32)
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

fn teleport(current_pos: i32, max_pos: i32) -> i32 {
    if current_pos >= 0 && current_pos < max_pos {
        return current_pos;
    } else if current_pos < 0 {
        return teleport(max_pos + current_pos, max_pos);
    } else {
        return teleport(current_pos - max_pos, max_pos);
    }
}

fn get_safety_factor(robots: Vec<Robot>) -> i32 {
    let mut quadrant_count = (0,0,0,0);
    for robot in robots.into_iter() {
        if robot.position.0 >= 0 && robot.position.0 < WIDTH/2 {
            if robot.position.1 >= 0 && robot.position.1 < HEIGHT/2 {
                quadrant_count.0 += 1;
            } else if robot.position.1 > HEIGHT/2 && robot.position.1 < HEIGHT {
                quadrant_count.1 += 1;
            }
        } else if robot.position.0 > WIDTH/2 && robot.position.0 < WIDTH {
            if robot.position.1 >= 0 && robot.position.1 < HEIGHT/2 {
                quadrant_count.2 += 1;
            } else if robot.position.1 > HEIGHT/2 && robot.position.1 < HEIGHT {
                quadrant_count.3 += 1;
            }
        }
    }

    return quadrant_count.0*quadrant_count.1*quadrant_count.2*quadrant_count.3;
}

fn visualise_graph(robots: Vec<Robot>) {
    let mut graph: Vec<Vec<u32>> = vec![vec![0; 101]; 103];

    for robot in robots.into_iter() {
        graph[robot.position.1 as usize][robot.position.0 as usize] += 1;
    }

    for line in graph.iter() {
        for point in line.iter() {
            if *point == 0 {
                print!(".");
            } else {
                print!("{}", point);
            }
        }
        println!();
    }
}

pub fn main(input_string: String) -> (String, String) {
    let re = Regex::new(r"p=(?<pos_x>(-)?[0-9]+),(?<pos_y>(-)?[0-9]+) v=(?<vel_x>(-)?[0-9]+),(?<vel_y>(-)?[0-9]+)").unwrap();
    let robots: Vec<Robot> = re.captures_iter(&input_string).map(|m| {
        let new_robot = Robot{
            position: (m.name("pos_x").unwrap().as_str().parse::<i32>().unwrap(), m.name("pos_y").unwrap().as_str().parse::<i32>().unwrap()),
            velocity: (m.name("vel_x").unwrap().as_str().parse::<i32>().unwrap(), m.name("vel_y").unwrap().as_str().parse::<i32>().unwrap())
        };
        return new_robot
    }).collect();

    let mut robots_p1 = robots.clone();
    for _ in 0..100 {
        for robots_p1 in robots_p1.iter_mut() {
            robots_p1.position.0 = teleport(robots_p1.position.0 + robots_p1.velocity.0, WIDTH);
            robots_p1.position.1 = teleport(robots_p1.position.1 + robots_p1.velocity.1, HEIGHT);
        }
    }

    let mut robots_p2 = robots.clone();
    let mut min_safety = (0, 0, robots_p2.clone());
    for n in 1..10000 {
        for robots_p2 in robots_p2.iter_mut() {
            robots_p2.position.0 = teleport(robots_p2.position.0 + robots_p2.velocity.0, WIDTH);
            robots_p2.position.1 = teleport(robots_p2.position.1 + robots_p2.velocity.1, HEIGHT);
        }
        let safety_factor = get_safety_factor(robots_p2.clone());
        if min_safety.0 == 0 || min_safety.0 > safety_factor {
            min_safety = (safety_factor, n, robots_p2.clone());
        }
    }

    visualise_graph(min_safety.2);

    (get_safety_factor(robots_p1).to_string(), min_safety.1.to_string())
}
