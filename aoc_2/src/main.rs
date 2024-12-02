use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let path: &str = "AoC_2.txt";

    let file: File = File::open(&path)?;
    let reader: BufReader<File> = BufReader::new(file);

    let mut safe_reports = 0;

    for line in reader.lines() {
        let line: String = line?;
        let row: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();

        if is_safe_with_dampener(&row) {
            safe_reports += 1;
        }
    }

    println!("Safe Reports: {}", safe_reports);
    Ok(())
}

fn is_safe(report: &Vec<i32>) -> bool {
    let _n = report.len();
    let increasing = report.windows(2).all(|w| 1 <= w[1] - w[0] && w[1] - w[0] <= 3);
    let decreasing = report.windows(2).all(|w| 1 <= w[0] - w[1] && w[0] - w[1] <= 3);
    increasing || decreasing
}

fn is_safe_with_dampener(report: &Vec<i32>) -> bool {
    if is_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut temp_report = report.clone();
        temp_report.remove(i);
        if is_safe(&temp_report) {
            return true;
        }
    }

    false
}