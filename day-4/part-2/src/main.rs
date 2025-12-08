mod directions;
use crate::directions::Direction;
use crate::directions::Point;
// use crate::directions::Direction::*;
use reqwest::blocking::Client;
use std::env;
use std::error::Error;

fn find_mas(map: &Vec<Vec<char>>, x: usize, y: usize) -> usize {
    let neighbourgs = vec![
        map[x - 1][y - 1],
        map[x - 1][y + 1],
        map[x + 1][y - 1],
        map[x + 1][y + 1],
    ];
    let mut m_count = 0;
    let mut s_count = 0;
    for cell in &neighbourgs {
        match cell {
            'M' => m_count += 1,
            'S' => s_count += 1,
            _ => return 0,
        };
    }
    if m_count == s_count
        && map[x - 1][y - 1] != map[x + 1][y + 1]
        && map[x + 1][y - 1] != map[x - 1][y + 1] {

        println!("x = {x} | y = {y}");
        // dot_map[x - 1][y - 1] = map[x - 1][y - 1];
        // dot_map[x + 1][y - 1] = map[x + 1][y - 1];
        // dot_map[x + 1][y + 1] = map[x + 1][y + 1];
        // dot_map[x - 1][y + 1] = map[x - 1][y + 1];
        // dot_map[x][y] = map[x][y];
        return 1;
    }
    // println!(
    //     "A in x = {} y = {} | m_count = {m_count} s_count = {s_count}\nneighbourgs = {:?}",
    //     x, y, neighbourgs
    // );
    return 0;
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input()?;
    let map: Vec<Vec<char>> = input
        .iter()
        .map(|l| l.chars().collect())
        .collect();
    // let map = get_map(input);
    // let map: Vec<Vec<char>> = vec![
    //     "MMMSXXMASM".chars().collect(),
    //     "MSAMXMSMSA".chars().collect(),
    //     "AMXSXMAAMM".chars().collect(),
    //     "MSAMASMSMX".chars().collect(),
    //     "XMASAMXAMM".chars().collect(),
    //     "XXAMMXXAMA".chars().collect(),
    //     "SMSMSASXSS".chars().collect(),
    //     "SAXAMASAAA".chars().collect(),
    //     "MAMMMXMMMM".chars().collect(),
    //     "MXMXAXMASX".chars().collect(),
    // ];
    // let mut dot_map: Vec<Vec<char>> = vec![
    //     "..........".chars().collect(),
    //     "..........".chars().collect(),
    //     "..........".chars().collect(),
    //     "..........".chars().collect(),
    //     "..........".chars().collect(),
    //     "..........".chars().collect(),
    //     "..........".chars().collect(),
    //     "..........".chars().collect(),
    //     "..........".chars().collect(),
    //     "..........".chars().collect(),
    // ];

    // println!("=========================");
    // for l in &map {
    //     println!("{:?}", l);
    // }
    let mut count = 0;
    for x in 1..map.len() - 1 {
        for y in 1..map[x].len() - 1 {
            count += match map[x][y] {
                'A' => find_mas(&map, x, y),
                _ => 0,
            };
        }
    }
    // for l in dot_map {
    //     println!("{:?}", l);
    // }
    println!("passowrd = {count}");
    //
    // let mut count: u64 = 0;
    // let mut x = 0;
    // loop {
    //     let mut y = 0;
    //     loop {
    //         let point = Point {x: x, y: y};
    //         for dir in Direction::ALL {
    //             if find_mas(&map, &point, 'M', &dir) > 0 && (
    //                 find_mas(&map, &point.delta(dir).delta(dir.left()),'M', &dir.right()) > 0 ||
    //                 find_mas(&map, &point.delta(dir).delta(dir.right()), 'M', &dir.left()) > 0
    //                 ) {
    //                 count += 1;
    //             }
    //         }
    //         if y == map[x].len() - 1 {
    //             break;
    //         }
    //         y += 1;
    //     }
    //     if x == map.len() - 1 {
    //         break;
    //     }
    //     x += 1;
    // }

    // println!("=========================");
    // for l in map {
    //     println!("{:?}", l);
    // }
    Ok(())
}

fn get_input() -> Result<Vec<String>, Box<dyn Error>> {
    dotenv::from_path("../../.env").ok();
    let session_cookie = env::var("AOC_SESSION")?;

    let url = "https://adventofcode.com/2024/day/4/input";

    let client = Client::new();
    let response = client
        .get(url)
        .header("Cookie", format!("session={}", session_cookie))
        .send()?
        .text()?;

    let lines: Vec<String> = response.lines().map(|l| l.to_string()).collect();

    Ok(lines)
}
