use regex::Regex;

// #[derive(Debug)]
struct Machine {
    button_a: (i32, i32),
    button_b: (i32, i32),
    prize: (i32, i32)
}

fn solve_machine(machine: Machine) -> i32 {
    let mut min_tokens_needed = 0;

    let mut matrix: Vec<Vec<(i32, i32)>> = vec![vec![(0, 0); 101]; 101];

    for i in 0..101 {
        matrix[i][0] = (machine.button_a.0 * i as i32, machine.button_a.1 * i as i32);
        if matrix[i][0].0 > machine.prize.0 || matrix[i][0].1 > machine.prize.1 {
            break;
        }
    }

    for j in 0..101 {
        matrix[0][j] = (machine.button_b.0 * j as i32, machine.button_b.1 * j as i32);
        if matrix[0][j].0 > machine.prize.0 || matrix[0][j].1 > machine.prize.1 {
            break;
        }
    }

    for i in 1..101 {
        for j in 1..101 {
            matrix[i][j] = (matrix[i][j-1].0 + machine.button_b.0, matrix[i][j-1].1 + machine.button_b.1);
            if matrix[i][j].0 > machine.prize.0 || matrix[i][j].1 > machine.prize.1 {
                break;
            }
            if matrix[i][j].0 == machine.prize.0 && matrix[i][j].1 == machine.prize.1 {
                let tokens_needed = i * 3 + j;
                if min_tokens_needed == 0 || min_tokens_needed > tokens_needed as i32 {
                    min_tokens_needed = tokens_needed as i32;
                }
            }
        }
    }

    min_tokens_needed
}

pub fn main(input_string: String) -> (String, String) {
    let re = Regex::new(r"Button A: X\+(?<but_a_x>[0-9]+), Y\+(?<but_a_y>[0-9]+)\nButton B: X\+(?<but_b_x>[0-9]+), Y\+(?<but_b_y>[0-9]+)\nPrize: X=(?<prize_x>[0-9]+), Y=(?<prize_y>[0-9]+)").unwrap();
    let machines: Vec<Machine> = re.captures_iter(&input_string).map(|m| {
        let new_machine = Machine{
            button_a: (m.name("but_a_x").unwrap().as_str().parse::<i32>().unwrap(), m.name("but_a_y").unwrap().as_str().parse::<i32>().unwrap()),
            button_b: (m.name("but_b_x").unwrap().as_str().parse::<i32>().unwrap(), m.name("but_b_y").unwrap().as_str().parse::<i32>().unwrap()),
            prize: (m.name("prize_x").unwrap().as_str().parse::<i32>().unwrap(), m.name("prize_y").unwrap().as_str().parse::<i32>().unwrap())
        };
        return new_machine
    }).collect();

    let mut total_tokens_needed = 0;
    for machine in machines.into_iter() {
        total_tokens_needed += solve_machine(machine);
    }

    (total_tokens_needed.to_string(), 0.to_string())
}
