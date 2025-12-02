
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

fn get_todo_sections(input: &Vec<String>) -> Vec<String> {
    let mut do_op = true;
    let line = input.join("");
    let mut res = Vec::new();
    let mut start = 0;
    while start < line.len() {
        if do_op {
            if let Some(dont_pos) = line[start..].find("don't()") {
                res.push(line[start..start + dont_pos].to_string());
                start = start + dont_pos + 7;
                do_op = false;
            } else {
                res.push(line[start..line.len()].to_string());
                break;
            }
        } else if !do_op {
            if let Some(do_pos) = line[start..].find("do()") {
                start = start + do_pos + 4;
                do_op = true;
            } else {
                break;
            }
        }
    }
    res
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input()?;
    let slices = get_todo_sections(&input);
    let mut res = 0;
    let pattern = "mul(";
    for line in &slices {
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

