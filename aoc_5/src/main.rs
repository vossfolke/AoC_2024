use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let file_path = "AoC_5_rules.txt";
    let update_path = "AoC_5_updates.txt";
    let hashmap = read_file_to_hashmap(file_path)?;
    let updates = read_file_to_vec(update_path)?;

    for (key, value) in &hashmap {
        println!("{}: {:?}", key, value);
    }

    for update in &updates {
        println!("{:?}", update);
    }

    let mut result_sum = 0;
    let mut result_fixed_updates = 0;
    let mut incorrect_updates: Vec<Vec<i32>> = Vec::new();
    for update in &updates {
        if correct_order(&hashmap, &update) {
            result_sum += update[update.len() / 2];
        } else {
            incorrect_updates.push(update.clone());
        }
    }
    for incorrect_update in incorrect_updates{
        result_fixed_updates += sort_update(&incorrect_update, &hashmap);
    }

    println!("Result is: {}", result_sum);

    println!("Result of fixed updates is: {}", result_fixed_updates);

    

    Ok(())
}

fn read_file_to_vec(file_path: &str) -> io::Result<Vec<Vec<i32>>> {
    let mut updates: Vec<Vec<i32>> = Vec::new();
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        //let update: Result<Vec<i32>, _> = line.split(',').map(|v| v.parse::<i32>()).collect();
        if let Ok(update) = line.split(',').map(|v| v.parse::<i32>()).collect() {
            updates.push(update);
        }
    }
    Ok(updates)
}

fn read_file_to_hashmap(file_path: &str) -> io::Result<HashMap<i32, Vec<i32>>> {
    let mut hashmap: HashMap<i32, Vec<i32>> = HashMap::new();
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() == 2 {
            let key: i32 = parts[0].trim().parse().unwrap();
            let value: i32 = parts[1].trim().parse().unwrap();
            hashmap.entry(key).or_insert_with(Vec::new).push(value);
        }
    }
    Ok(hashmap)
}

fn correct_order (rules: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> bool {
    let reverse_update: Vec<i32> = update.iter().rev().cloned().collect();
    for i_key in 0..reverse_update.len() {
        let key = reverse_update[i_key];
        if let Some(rule)    = rules.get(&key){
            for &value in rule {
                if reverse_update[i_key+1..].into_iter().any(|&num| num==value) {
                   println!("{:?}", &reverse_update[i_key+1..]);
                   return false;
                }
            }
        }   
    }
    println!("########## {} ########", true);

    return true;
}

fn sort_update(unsorted: &Vec<i32>, rules: &HashMap<i32, Vec<i32>>) -> i32 {
    let mut safe_update = make_correct_order(rules, unsorted);
    safe_update[safe_update.len() / 2]
}

fn make_correct_order(rules: &HashMap<i32, Vec<i32>>, unsorted: &Vec<i32>) -> Vec<i32> {
    let mut in_degree = HashMap::new();
    let mut graph = HashMap::new();

    for &page in unsorted {
        in_degree.insert(page, 0);
        graph.insert(page, Vec::new());
    }

    for (&key, values) in rules {
        if !unsorted.contains(&key) {
            continue;
        }
        for &value in values {
            if !unsorted.contains(&value) {
                continue;
            }
            graph.get_mut(&key).unwrap().push(value);
            *in_degree.get_mut(&value).unwrap() += 1;
        }
    }

    let mut queue = Vec::new();
    for (&node, &degree) in &in_degree {
        if degree == 0 {
            queue.push(node);
        }
    }

    let mut sorted = Vec::new();
    while let Some(node) = queue.pop() {
        sorted.push(node);
        for &neighbor in &graph[&node] {
            let degree = in_degree.get_mut(&neighbor).unwrap();
            *degree -= 1;
            if *degree == 0 {
                queue.push(neighbor);
            }
        }
    }

    sorted
}