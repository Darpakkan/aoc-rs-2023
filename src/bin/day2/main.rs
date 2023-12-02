use std::env;
use std::fs;

const max_red: usize = 12;
const max_green: usize = 13;
const max_blue: usize = 14;

fn is_valid(line: String) -> bool {
    let line = line.split(':').nth(1).unwrap();
    let games: Vec<String> = line.split(";").map(|a| a.trim().to_string()).collect();
    let mut red = 0;
    let mut blue = 0;
    let mut red = 0;
    for game in games {
        let balls: Vec<String> = game.split(',').map(|a| a.trim().to_string()).collect();
        for ball in balls {
            let sample: Vec<String> = ball.split(' ').map(|a| a.to_string()).collect();
            let no: usize = sample[0].parse::<usize>().unwrap();
            let color: &str = &sample[1].as_ref();
            match color {
                "red" => {
                    if no > max_red {
                        return false;
                    }
                },
                "blue" => {
                    if no > max_blue {
                        return false;
                    }
                },
                "green" =>{
                    if no > max_green {
                        return false;
                    }
                },
                _ => return false,
            }
        }
    }
    true
}

fn find_res_line(line: String) -> usize {
    let line = line.split(':').nth(1).unwrap();
    let games: Vec<String> = line.split(";").map(|a| a.trim().to_string()).collect();
    let mut min_red = 0;
    let mut min_blue = 0;
    let mut min_green = 0;
    let mut red = 0;
    let mut blue = 0;
    let mut red = 0;
    for game in games {
        let balls: Vec<String> = game.split(',').map(|a| a.trim().to_string()).collect();
        for ball in balls {
            let sample: Vec<String> = ball.split(' ').map(|a| a.to_string()).collect();
            let no: usize = sample[0].parse::<usize>().unwrap();
            let color: &str = &sample[1].as_ref();
            match color {
                "red" => {
                    if min_red < no {
                        min_red = no;
                    }
                },
                "blue" => {
                    if min_blue < no {
                        min_blue = no;
                    }
                },
                "green" =>{
                    if min_green < no {
                        min_green = no;
                    }
                },
                _ => return 1,
            }
        }
    }
    return min_red*min_blue*min_green;
}

fn main() {
    let contents = fs::read_to_string("src/bin/day2/input.txt")
        .expect("Should have been able to read the file");

    let lines: Vec<String> = contents.split('\n').map(|a| String::from(a)).collect();

    // Part A

    let mut sum: usize = 0;

    let mut idx = 1;
    for line in lines.clone() {
        if is_valid(line) {
            sum += idx;
        }
        idx+=1;
    }

    println!("sum: {}", sum);

    // part B
    let mut sum: usize = 0;
    
    for line in lines {
        sum += find_res_line(line);
    }

    println!("sum: {}", sum);
}