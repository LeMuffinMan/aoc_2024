
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

fn sort_list(list: &String, rules_map: &HashMap::<String, Vec<String>>) -> usize {
    let mut pages: Vec<&str> = list.split(',').collect();
    let mut sorted_list = pages.clone();
    println!("{:?}", pages);
    while let Some(page) = pages.pop() {
        sorted_list.pop();
        let forbidden_pages = &rules_map[page];
        for j in 0..sorted_list.len() {
            if forbidden_pages.contains(&sorted_list[j].to_string()) {
                sorted_list.insert(j, page);
            }
        }
    }
    println!("{:?}", sorted_list);
    sorted_list[pages.len() / 2].parse().unwrap()
}

// 4623 -> too high

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input()?;
    let (rules, updates) = extract(input);
    let rules_map = get_rules_map(rules);
    let mut to_sort_updates = Vec::new();
    for list in updates {
        if !check_list(&list, &rules_map) {
            to_sort_updates.push(list.clone());
        }
    }
    let mut count: usize = 0;
    for list in to_sort_updates {
        count += sort_list(&list, &rules_map);
    }
    println!("password = {count}");
    Ok(())
}

fn get_input() -> Result<Vec<String>, Box<dyn Error>> {
    dotenv::from_path("../../.env").ok();
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

