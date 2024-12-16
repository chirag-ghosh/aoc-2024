use regex::Regex;

#[derive(Debug)]
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

pub fn main(input_string: String) -> (String, String) {
    let re = Regex::new(r"p=(?<pos_x>(-)?[0-9]+),(?<pos_y>(-)?[0-9]+) v=(?<vel_x>(-)?[0-9]+),(?<vel_y>(-)?[0-9]+)").unwrap();
    let mut robots: Vec<Robot> = re.captures_iter(&input_string).map(|m| {
        let new_robot = Robot{
            position: (m.name("pos_x").unwrap().as_str().parse::<i32>().unwrap(), m.name("pos_y").unwrap().as_str().parse::<i32>().unwrap()),
            velocity: (m.name("vel_x").unwrap().as_str().parse::<i32>().unwrap(), m.name("vel_y").unwrap().as_str().parse::<i32>().unwrap())
        };
        return new_robot
    }).collect();

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.position.0 = teleport(robot.position.0 + robot.velocity.0, WIDTH);
            robot.position.1 = teleport(robot.position.1 + robot.velocity.1, HEIGHT);
        }
    }

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

    ((quadrant_count.0*quadrant_count.1*quadrant_count.2*quadrant_count.3).to_string(), 0.to_string())
}
