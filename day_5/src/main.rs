use std::{collections::HashSet, fs};

fn main() {
    let input_string = fs::read_to_string("./src/input.txt").unwrap();
    let mut orders: HashSet<(i32,i32)> = HashSet::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    for line in input_string.lines().into_iter() {
        if line.contains("|") {
            let line_split = line.split("|").map(|value| value.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            orders.insert((line_split[0],line_split[1]));
        } else if line.contains(",") {
            let line_split = line.split(",").map(|value| value.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            updates.push(line_split);
        }
    }

    let mut sum = 0;
    for update in updates.iter() {
        let mut is_correct = true;
        for i in 0..update.len() {
            for j in (i+1)..update.len() {
                if orders.contains(&(update[j],update[i])) {
                    is_correct = false;
                    break;
                }
            }
            if !is_correct {
                break;
            }
        }

        if is_correct {
            sum += update[(update.len()-1)/2];
        }
    }

    println!("Sum is: {}", sum);
}
