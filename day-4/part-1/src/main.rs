mod directions;
use crate::directions::Direction;
use crate::directions::Direction::*;
use std::env;
use reqwest::blocking::Client;
use std::error::Error;

fn get_map(input: Vec<String>) -> Vec<Vec<char>> {
    let mut map = Vec::new();
    for line in input {
        let mut buf = Vec::new();
        for char in line.chars() {
            buf.push(char);
        }
        map.push(buf);
    }
    map
}


fn find_xmas(map: &Vec<Vec<char>>, x: usize, y: usize, cha: char, dir: Direction, dot_map: &mut Vec<Vec<char>>) -> u64 {
    if x == map.len() || y == map[x].len() {
        return 0;
    } 
    if map[x as usize][y as usize] == cha {
        match map[x as usize][y as usize] {
            'X' => {
                let mut count = 0;
                let res = find_xmas(map, x, y + 1, 'M', East, dot_map);
                if res > 0 {
                    count += res;
                    dot_map[x][y] = map[x][y];
                }
                if y > 0 {
                    let res = find_xmas(map, x, y - 1, 'M', West, dot_map);
                    if res > 0 {
                        count += res;
                        dot_map[x][y] = map[x][y];
                    }
                }
                let res = find_xmas(map, x + 1, y, 'M', North, dot_map);
                if res > 0 {
                    count += res;
                    dot_map[x][y] = map[x][y];
                }
                if x > 0 {
                    let res = find_xmas(map, x - 1, y, 'M', South, dot_map);
                    if res > 0 {
                        count += res;
                        dot_map[x][y] = map[x][y];
                    }
                }
                let res = find_xmas(map, x + 1, y + 1, 'M', NorthEast, dot_map);
                if res > 0 {
                    count += res;
                    dot_map[x][y] = map[x][y];
                }
                if x > 0 {
                    let res = find_xmas(map, x - 1, y + 1, 'M', SouthEast, dot_map);
                    if res > 0 {
                        count += res;
                        dot_map[x][y] = map[x][y];
                    }
                }
                if y > 0 {
                    let res = find_xmas(map, x + 1, y - 1, 'M', NorthWest, dot_map);
                    if res > 0 {
                        count += res;
                        dot_map[x][y] = map[x][y];
                    }
                }
                if x > 0  && y > 0 {
                    let res = find_xmas(map, x - 1, y - 1, 'M', SouthWest, dot_map);
                    if res > 0 {
                        count += res;
                        dot_map[x][y] = map[x][y];
                    }
                }
                return count;
            },
            'M' => {
                match dir {
                    North => {
                        if find_xmas(map, x + 1, y, 'A', dir, dot_map) != 0 {
                            dot_map[x][y] = map[x][y]; 
                            return 1;
                        }
                        else {
                            return 0;
                        }
                    },
                    East => {
                        if find_xmas(map, x, y + 1, 'A', dir, dot_map) != 0 {
                            dot_map[x][y] = map[x][y]; 
                            return 1;
                        }
                        else {
                            return 0;
                        }
                    }
                    South => {
                        if x > 0 && find_xmas(map, x - 1, y, 'A', dir, dot_map) != 0 {
                            dot_map[x][y] = map[x][y]; 
                            return 1;
                        }
                        else {
                            return 0;
                        }
                    },
                    West => {
                        if y > 0 && find_xmas(map, x, y - 1, 'A', dir, dot_map) != 0 {
                            dot_map[x][y] = map[x][y]; 
                            return 1;
                        }
                        else {
                            return 0;
                        }
                    }
                    NorthEast => {
                        if find_xmas(map, x + 1, y + 1, 'A', dir, dot_map) != 0 {
                            dot_map[x][y] = map[x][y]; 
                            return 1;
                        }
                        else {
                            return 0;
                        }
                    },
                    SouthEast => {
                        if x > 0 && find_xmas(map, x - 1, y + 1, 'A', dir, dot_map) != 0 {
                            dot_map[x][y] = map[x][y]; 
                            return 1;
                        }
                        else {
                            return 0;
                        }
                    },
                    NorthWest => {
                        if y > 0 && find_xmas(map, x + 1, y - 1, 'A', dir, dot_map) != 0 {
                            dot_map[x][y] = map[x][y]; 
                            return 1;
                        }
                        else {
                            return 0;
                        }
                    }
                    SouthWest => {
                        if x > 0 && y > 0 && find_xmas(map, x - 1, y - 1, 'A', dir, dot_map) != 0 {
                            dot_map[x][y] = map[x][y]; 
                            return 1;
                        }
                        else {
                            return 0;
                        }
                    }
                }

            },
            'A' => {
                match dir {
                    North => {
                        if find_xmas(map, x + 1, y, 'S', dir, dot_map) != 0 {
                            dot_map[x][y] = map[x][y]; 
                            return 1;
                        }
                        else {
                            return 0;
                        }
                    },
                    East => {
                        if find_xmas(map, x, y + 1, 'S', dir, dot_map) != 0 {
                            dot_map[x][y] = map[x][y]; 
                            return 1;
                        }
                        else {
                            return 0;
                        }
                    }
                    South => {
                        if x > 0 && find_xmas(map, x - 1, y, 'S', dir, dot_map) != 0 {
                            dot_map[x][y] = map[x][y]; 
                            return 1;
                        }
                        else {
                            return 0;
                        }
                    },
                    West => {
                        if y > 0 && find_xmas(map, x, y - 1, 'S', dir, dot_map) != 0 {
                            dot_map[x][y] = map[x][y]; 
                            return 1;
                        }
                        else {
                            return 0;
                        }
                    }
                    NorthEast => {
                        if find_xmas(map, x + 1, y + 1, 'S', dir, dot_map) != 0 {
                            dot_map[x][y] = map[x][y]; 
                            return 1;
                        }
                        else {
                            return 0;
                        }
                    },
                    SouthEast => {
                        if x > 0 && find_xmas(map, x - 1, y + 1, 'S', dir, dot_map) != 0 {
                            dot_map[x][y] = map[x][y]; 
                            return 1;
                        }
                        else {
                            return 0;
                        }
                    },
                    NorthWest => {
                        if y > 0 && find_xmas(map, x + 1, y - 1, 'S', dir, dot_map) != 0 {
                            dot_map[x][y] = map[x][y]; 
                            return 1;
                        }
                        else {
                            return 0;
                        }
                    }
                    SouthWest => {
                        if y > 0 && x > 0 && find_xmas(map, x - 1, y - 1, 'S', dir, dot_map) != 0 {
                            dot_map[x][y] = map[x][y]; 
                            return 1;
                        }
                        else {
                            return 0;
                        }
                    }
                }

            },
            'S' => {
                dot_map[x][y] = map[x][y]; 
                return 1;
            }
            _ => { unreachable!() },
        }
    } else {
        return 0;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input()?;
    let map = get_map(input);
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
    let mut dot_map = map.clone();
    let mut count: u64 = 0;
    let mut x = 0;
    loop {
        let mut y = 0;
        loop {
            count += find_xmas(&map, x as usize, y as usize, 'X', North, &mut dot_map);
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

fn get_input() -> Result<Vec<String>, Box<dyn Error>> {
    dotenv::from_path("../.env").ok();
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

