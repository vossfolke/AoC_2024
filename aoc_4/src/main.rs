use std::fs::File;
use std::io::{self, BufRead, BufReader};
use regex::Regex;

fn main() -> io::Result<()> {
    let path = "AoC_4.txt";
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let re_xmas = Regex::new(r"XMAS").unwrap();
    let re_samx = Regex::new(r"SAMX").unwrap();

    let mut xmas_count = 0;

    let mut columns = vec![String::new(); lines[0].len()];

    for line in &lines {
        for (i, c) in line.chars().enumerate() {
            columns[i].push(c);
        }
        // XMAS in lines
        xmas_count += re_xmas.find_iter(line).count();
        xmas_count += re_samx.find_iter(line).count();
    }

    // Diagonal checks
    xmas_count += count_diagonals(&lines, 1, 1);
    xmas_count += count_diagonals(&lines, 1, -1);

    for column in &columns {
        xmas_count += re_xmas.find_iter(column).count();
        xmas_count += re_samx.find_iter(column).count(); 
    }

    println!("The sum of all XMAS appearances: {}", xmas_count);

    let count_x_mas = check_x_mas(&lines);

    println!("The sum of all X-MAS appearances: {}", count_x_mas);

    Ok(())
}

fn count_diagonals(lines: &Vec<String>, dx: isize, dy: isize) -> usize {
    let mut count = 0;
    let height = lines.len() as isize;
    let width = lines[0].len() as isize;

    for i in 0..height {
        for j in 0..width {
            if check_word(&lines, i, j, dx, dy, "XMAS") {
                count += 1;
            }
            if check_word(&lines, i, j, dx, dy, "SAMX") {
                count += 1;
            }
        }
    }

    count
}

fn check_word(lines: &Vec<String>, x: isize, y: isize, dx: isize, dy: isize, word: &str) -> bool {
    let height = lines.len() as isize;
    let width = lines[0].len() as isize;

    for (i, ch) in word.chars().enumerate() {
        let nx = x + i as isize * dx;
        let ny = y + i as isize * dy;

        if nx < 0 || ny < 0 || nx >= height || ny >= width {
            return false;
        }

        if lines[nx as usize].chars().nth(ny as usize) != Some(ch) {
            return false;
        }
    }

    true
}

fn check_x_mas (lines: &Vec<String>) -> usize {
    let mut count = 0;

    let height = lines.len() as isize;
    let width = lines[0].len() as isize;

    for i in 0..height {
        for j in 0..width {
            if check_word(&lines, i, j, 1, -1, "MAS") {
                if check_word(lines, i+2, j, -1, -1, "MAS") {
                    count += 1;
                } else if check_word(lines, i+2, j, -1, -1, "SAM"){
                    count += 1;
                }
            }
            if check_word(&lines, i, j, 1, -1, "SAM") {
                if check_word(lines, i+2, j, -1, -1, "MAS") {
                    count += 1;
                } else if check_word(lines, i+2, j, -1, -1, "SAM"){
                    count += 1;
                }
            }
        }
    }
    count
}