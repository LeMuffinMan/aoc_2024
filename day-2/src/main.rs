
use std::env;
use reqwest::blocking::Client;
use std::error::Error;


fn is_safe(report: &Vec<i32>, len: usize) -> usize {
    let mut i = 0;
    if report[0] < report[1] {
        loop {
            if i == len - 1 {
                return len;
            }
            if report[i] >= report[i + 1] || report[i + 1] - report[i] > 3 || report[i + 1] - report[i] < 1 {
                return i;
            }
            i += 1;
        }
    } else {
        loop {
            if i == len - 1 { 
                return len;
            }
            if report[i] <= report[i + 1] || report[i] - report[i + 1] > 3 || report[i] - report[i + 1] < 1 { 
                return i;
            } 
            i += 1;
        }
    }
}

fn is_tolerated(report: &Vec<i32>, index_to_remove: usize) -> bool {
    let mut v = report.clone();
    v.remove(index_to_remove);
    let len = v.len();
    if is_safe(&v, len) == len {
        return true;
    }
    return false;
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut count = 0;
    let input = get_input()?;
    for report in &input {
        let vec: Vec<i32> = report
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let len = vec.len();
        let res = is_safe(&vec, len);
        if res == len {
            count += 1;
        } else {
            let mut i = 0;
            loop {
                if i == vec.len() {
                    break;
                }
                if is_tolerated(&vec, i) {
                    count += 1;
                    break;
                }
                i += 1;
            }
        }
    }
    println!("password = {count}");
    Ok(())
}

fn get_input() -> Result<Vec<String>, Box<dyn Error>> {
    dotenv::from_path("../.env").ok();
    let session_cookie = env::var("AOC_SESSION")?;

    let url = "https://adventofcode.com/2024/day/2/input";

    let client = Client::new();
    let response = client
        .get(url)
        .header("Cookie", format!("session={}", session_cookie))
        .send()?
        .text()?;

    let lines: Vec<String> = response.lines().map(|l| l.to_string()).collect();

    Ok(lines)
}

