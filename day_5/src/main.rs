use std::{cmp::Ordering, collections::HashSet, fs};

fn check_order(orders: HashSet<(i32,i32)>, a:i32, b:i32) -> Ordering {
    if orders.contains(&(a,b)) {
        return Ordering::Less;
    } else if orders.contains(&(b,a)) {
        return Ordering::Greater;
    } else {
        return Ordering::Equal;
    }
}

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

    let mut incorrect_updates: Vec<Vec<i32>> = Vec::new();
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
        } else {
            incorrect_updates.push(update.to_owned());
        }
    }
    println!("Sum is: {}", sum);

    let mut corrected_sum = 0;
    for update in incorrect_updates.iter_mut() {
        update.sort_by(|a,b| check_order(orders.clone(), a.to_owned(), b.to_owned()));
        corrected_sum += update[(update.len()-1)/2];
    }
    println!("Corrected sum is: {}", corrected_sum);
}
