use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // Path to the file
    let path = "AoC_1.txt";
    // Open the file and handle potential errors
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    // Vectors to store the columns
    let mut first_column: Vec<i32> = Vec::new();
    let mut second_column: Vec<i32> = Vec::new();

    // Read each line from the file
    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        println!("Line {}: {:?}", index + 1, line); // Debugging output

        // Split the line by whitespace
        let columns: Vec<&str> = line.split_whitespace().collect();
        println!("Split Line {}: {:?}", index + 1, columns); // Debugging output

        if columns.len() >= 2 {
            // Parse and push into respective vectors
            if let Ok(first) = columns[0].trim().parse::<i32>() {
                first_column.push(first);
            } else {
                eprintln!(
                    "Failed to parse first column on line {}: {:?}",
                    index + 1,
                    columns[0]
                );
            }
            if let Ok(second) = columns[1].trim().parse::<i32>() {
                second_column.push(second);
            } else {
                eprintln!(
                    "Failed to parse second column on line {}: {:?}",
                    index + 1,
                    columns[1]
                );
            }
        } else {
            eprintln!("Insufficient columns on line {}: {:?}", index + 1, columns);
        }
    }

    (first_column, second_column) = sort_input(first_column.clone(), second_column.clone());

    println!("First column: {:?}", first_column);

    println!(
        "Total Distance is {}",
        total_distance(&first_column, &second_column)
    );

    println!(
        "Similarity Score {}",
        similarity_score(&first_column, &second_column)
    );

    Ok(())
}

// getting 2 lists of integer, sort them accending and adding the difference between the values from list 1 and 2

fn total_distance(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut result = 0;
    for index1 in 0..list1.len() {
        if list1[index1] <= list2[index1] {
            result = result + (list2.get(index1).unwrap() - list1.get(index1).unwrap());
        } else {
            result = result + (list1.get(index1).unwrap() - list2.get(index1).unwrap());
        }
    }
    result
}

fn sort_input(mut list1: Vec<i32>, mut list2: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
    list1.sort();
    list2.sort();
    (list1, list2)
}

fn similarity_score(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut result = Vec::new();

    for index1 in 0..list1.len() {
        let mut single_sim = 0;
        for index2 in 0..list2.len() {
            if list1[index1] == list2[index2] {
                single_sim += 1;
            }
        }
        result.push(single_sim * list1[index1]);
    }
    let mut similarity = result.iter().sum();
    similarity
}
