use regex::Regex;

#[derive(Clone, Copy, Debug)]
struct Machine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64)
}

fn solve(machine: Machine) -> i64 {
    let num_a = (machine.prize.0 * machine.button_b.1 - machine.prize.1 * machine.button_b.0) / (machine.button_a.0 * machine.button_b.1 - machine.button_a.1 * machine.button_b.0);
    if (machine.prize.0 * machine.button_b.1 - machine.prize.1 * machine.button_b.0) % (machine.button_a.0 * machine.button_b.1 - machine.button_a.1 * machine.button_b.0) != 0 {
        return 0;
    }

    let num_b = (machine.prize.0 - (num_a * machine.button_a.0)) / machine.button_b.0;
    if(machine.prize.0 - (num_a * machine.button_a.0)) % machine.button_b.0 != 0 {
        return 0;
    }

    return num_a * 3 + num_b;
}

pub fn main(input_string: String) -> (String, String) {
    let re = Regex::new(r"Button A: X\+(?<but_a_x>[0-9]+), Y\+(?<but_a_y>[0-9]+)\nButton B: X\+(?<but_b_x>[0-9]+), Y\+(?<but_b_y>[0-9]+)\nPrize: X=(?<prize_x>[0-9]+), Y=(?<prize_y>[0-9]+)").unwrap();
    let machines: Vec<Machine> = re.captures_iter(&input_string).map(|m| {
        let new_machine = Machine{
            button_a: (m.name("but_a_x").unwrap().as_str().parse::<i64>().unwrap(), m.name("but_a_y").unwrap().as_str().parse::<i64>().unwrap()),
            button_b: (m.name("but_b_x").unwrap().as_str().parse::<i64>().unwrap(), m.name("but_b_y").unwrap().as_str().parse::<i64>().unwrap()),
            prize: (m.name("prize_x").unwrap().as_str().parse::<i64>().unwrap(), m.name("prize_y").unwrap().as_str().parse::<i64>().unwrap())
        };
        return new_machine
    }).collect();

    let corrected_machines: Vec<Machine> = machines.clone().iter_mut().map(|machine| {
        machine.prize.0 += 10000000000000;
        machine.prize.1 += 10000000000000;

        machine.to_owned()
    }).collect();

    let mut total_tokens_needed = 0;
    for machine in machines.into_iter() {
        total_tokens_needed += solve(machine);
    }

    let mut total_corrected_tokens_needed = 0;
    for machine in corrected_machines.into_iter() {
        total_corrected_tokens_needed += solve(machine);
    }

    (total_tokens_needed.to_string(), total_corrected_tokens_needed.to_string())
}
