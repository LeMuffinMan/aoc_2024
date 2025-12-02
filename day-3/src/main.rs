
use std::env;
use reqwest::blocking::Client;
use std::error::Error;

fn extract_number(line: &str, delimiter: char) -> Option<i32> {
    let mut num_str = String::new();
    for c in line.chars() {
        if c.is_digit(10) {
            num_str.push(c);
        } else if c == delimiter {
            return num_str.parse().ok();
        } else {
            return None;
        }
    }
    None
}

fn extract_tuple(line: &str) -> Option<(i32, i32)> {
    if let Some(n1) = extract_number(line, ',') {
        println!("n1 = {n1}");
        let mut i = 0;
        for c in line.chars() {
            if c == ',' {
                i += 1;
                break;
            }
            if !c.is_digit(10) {
                return None;
            }
            i += 1;
        }
        if let Some(n2) = extract_number(&line[i..], ')') {
            println!("n2 = {n2}");
            return Some((n1, n2));
        }
    }
    return None;
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input()?;
    println!("{:?}", input);
    let mut res = 0;
    let pattern = "mul(";
    for line in &input {
        let mut start = 0;
        while let Some(pos) = line[start..].find(pattern) {
            let pat_start = start + pos;
            let s = &line[pat_start + 4..];
            if let Some((n1, n2)) = extract_tuple(s) {
                res += n1 * n2;
            }
            start = pat_start + 4;
        }
    }
    println!("res = {res}");
    Ok(())
}

fn get_input() -> Result<Vec<String>, Box<dyn Error>> {
    dotenv::from_path("../.env").ok();
    let session_cookie = env::var("AOC_SESSION")?;

    let url = "https://adventofcode.com/2024/day/3/input";

    let client = Client::new();
    let response = client
        .get(url)
        .header("Cookie", format!("session={}", session_cookie))
        .send()?
        .text()?;

    let lines: Vec<String> = response.lines().map(|l| l.to_string()).collect();

    Ok(lines)
}

