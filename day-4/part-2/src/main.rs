mod directions;
use crate::directions::Direction;
use crate::directions::Point;
// use crate::directions::Direction::*;
use std::env;
use reqwest::blocking::Client;
use std::error::Error;

fn find_mas(map: &Vec<Vec<char>>, point: &Point, target: char, dir: &Direction) -> usize {
    if point.x < 0 || point.y < 0 || point.x > map.len() || point.y > map[point.x].len() || map[point.x][point.y] != target {
        return 0;
    }
    match map[point.x][point.y] {
        'M' => find_mas(map, &point.delta(*dir), 'A', dir), 
        'A' => find_mas(map, &point.delta(*dir), 'S', dir),
        'S' => 1,
        _ => 0,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // let input = get_input()?;
    // let map = get_map(input);
    let map: Vec<Vec<char>> = vec![
        "MMMSXXMASM".chars().collect(),
        "MSAMXMSMSA".chars().collect(),
        "AMXSXMAAMM".chars().collect(),
        "MSAMASMSMX".chars().collect(),
        "XMASAMXAMM".chars().collect(),
        "XXAMMXXAMA".chars().collect(),
        "SMSMSASXSS".chars().collect(),
        "SAXAMASAAA".chars().collect(),
        "MAMMMXMMMM".chars().collect(),
        "MXMXAXMASX".chars().collect(),
    ];
    let mut dot_map = map.clone();
    let mut count: u64 = 0;
    let mut x = 0;
    loop {
        let mut y = 0;
        loop {
            let point = Point {x: x, y: y};
            for dir in Direction::ALL {
                if find_mas(&map, &point, 'M', &dir) > 0 && (
                    find_mas(&map, &point.delta(dir).delta(dir.left()),'M', &dir.right()) > 0 ||
                    find_mas(&map, &point.delta(dir).delta(dir.right()), 'M', &dir.left()) > 0
                    ) {
                    count += 1;
                }
            }
            if y == map[x].len() - 1 {
                break;
            }
            y += 1;
        }
        if x == map.len() - 1 {
            break;
        }
        x += 1;
    }
    for l in dot_map {
        println!("{:?}", l);
    }
    println!("=========================");
    for l in map {
        println!("{:?}", l);
    }
    println!("passowrd = {count}");
    Ok(())
}

// fn get_input() -> Result<Vec<String>, Box<dyn Error>> {
//     dotenv::from_path("../.env").ok();
//     let session_cookie = env::var("AOC_SESSION")?;
//
//     let url = "https://adventofcode.com/2024/day/4/input";
//
//     let client = Client::new();
//     let response = client
//         .get(url)
//         .header("Cookie", format!("session={}", session_cookie))
//         .send()?
//         .text()?;
//
//     let lines: Vec<String> = response.lines().map(|l| l.to_string()).collect();
//
//     Ok(lines)
// }

