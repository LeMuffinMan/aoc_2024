use std::env;
use reqwest::blocking::Client;
use std::error::Error;

mod lists;
use crate::lists::Lists;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = get_input()?;
    // println!("{:?}", input);

    let mut lists = Lists { left: Vec::new(), right: Vec::new() };
    lists.get_lists(input);
    // lists.print_lists();
    lists.sort_lists();
    // lists.print_lists();
    println!("get_similarity {:?}", lists.get_similarity());
    println!("get_dist_sum {:?}", lists.get_dist_sum());

    Ok(())
}

fn get_input() -> Result<Vec<String>, Box<dyn Error>> {
    dotenv::from_path("../.env").ok();
    let session_cookie = env::var("AOC_SESSION")?;

    let url = "https://adventofcode.com/2024/day/1/input";

    let client = Client::new();
    let response = client
        .get(url)
        .header("Cookie", format!("session={}", session_cookie))
        .send()?
        .text()?;

    let lines: Vec<String> = response.lines().map(|l| l.to_string()).collect();

    Ok(lines)
}
