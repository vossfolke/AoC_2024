use regex::Regex;
use std::fs::File;
use std::io::{self, BufReader, Read};

fn main() -> io::Result<()> {
    let path = "AoC_3.txt";
    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);

    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    let mul_pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let do_pattern = Regex::new(r"do\(\)").unwrap();
    let dont_pattern = Regex::new(r"don't\(\)").unwrap();

    let mut enabled = true;
    let mut total_sum = 0;

    let mut index = 0;
    while index < content.len() {
        if let Some(do_match) = do_pattern.find(&content[index..]) {
            if do_match.start() == 0 {
                enabled = true;
                index += do_match.end();
                continue;
            }
        }

        if let Some(dont_match) = dont_pattern.find(&content[index..]) {
            if dont_match.start() == 0 {
                enabled = false;
                index += dont_match.end();
                continue;
            }
        }

        if let Some(mul_match) = mul_pattern.captures(&content[index..]) {
            if mul_match.get(0).unwrap().start() == 0 && enabled {
                let x: i32 = mul_match[1].parse().unwrap();
                let y: i32 = mul_match[2].parse().unwrap();
                total_sum += x * y;
                index += mul_match.get(0).unwrap().end();
                continue;
            }
        }

        index += 1;
    }

    println!("The sum of all enabled multiplications is: {}", total_sum);

    Ok(())
}