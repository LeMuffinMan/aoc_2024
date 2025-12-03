
use std::env;
use reqwest::blocking::Client;
use std::error::Error;
use std::collections::HashMap;

fn extract(input: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut switch = false;
    let mut rules = Vec::new();
    let mut updates = Vec::new();
    for s in input {
        if s == "" {
            switch = true;
            continue;
        }
        if !switch {
            rules.push(s); 
        } else {
            updates.push(s);
        }
    }
    (rules, updates)
}

fn check_list(list: &String, rules_map: &HashMap::<String, Vec<String>>) -> bool {
    let pages: Vec<&str> = list.split(',').collect();
    let len = pages.len();
    for i in (0..len).rev() {
        let forbidden_pages = &rules_map[pages[i]];
        for j in (0..i).rev() {
            if forbidden_pages.contains(&pages[j].to_string()) {
                return false;
            }
        }
    }
    true
}

fn get_rules_map(rules: Vec<String>) -> HashMap::<String, Vec<String>> {
    let mut map = HashMap::<String, Vec<String>>::new();
    for pair in rules {
        if let Some((key, value)) = pair.split_once('|') {
            map.entry(key.to_string())
                .or_insert(Vec::new())
                .push(value.to_string());
        }
    }
    map
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input()?;
    let (rules, updates) = extract(input);
    let rules_map = get_rules_map(rules);
    println!("{:?}", rules_map);
    println!("{:?}", updates);
    let mut validated_updates = Vec::new();
    for list in updates {
        if check_list(&list, &rules_map) {
            validated_updates.push(list);
        }
    }
    let mut count: usize = 0;
    for list in validated_updates {
        let digits: Vec<&str> = list.split(',').collect();
        count += digits[digits.len() / 2].parse::<usize>().unwrap();
    }
    println!("password = {count}");
    Ok(())
}

fn get_input() -> Result<Vec<String>, Box<dyn Error>> {
    dotenv::from_path("../.env").ok();
    let session_cookie = env::var("AOC_SESSION")?;

    let url = "https://adventofcode.com/2024/day/5/input";

    let client = Client::new();
    let response = client
        .get(url)
        .header("Cookie", format!("session={}", session_cookie))
        .send()?
        .text()?;

    let lines: Vec<String> = response.lines().map(|l| l.to_string()).collect();

    Ok(lines)
}

