#!/bin/bash

if [ -z $1 ]; then
    echo "Usage: ./gen_day <day-number>"
    exit 1
fi

NAME="day-$1" 
cargo new $NAME

echo "
use std::env;
use reqwest::blocking::Client;
use std::error::Error;

mod dial;
use dial::Dial;

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input()?;

    Ok(())
}

fn get_input() -> Result<Vec<String>, Box<dyn Error>> {
    dotenv::from_path(\"../.env\").ok();
    let session_cookie = env::var(\"AOC_SESSION\")?;

    let url = \"https://adventofcode.com/2024/day/$1/input\";

    let client = Client::new();
    let response = client
        .get(url)
        .header(\"Cookie\", format!(\"session={}\", session_cookie))
        .send()?
        .text()?;

    let lines: Vec<String> = response.lines().map(|l| l.to_string()).collect();

    Ok(lines)
}
" >> $NAME/src/main.rs
