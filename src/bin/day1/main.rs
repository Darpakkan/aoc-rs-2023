use std::env;
use std::fs;
use std::collections::HashMap;



fn main() {

    // PART A

    let contents = fs::read_to_string("src/bin/day1/input.txt")
        .expect("Should have been able to read the file");

    let lines: Vec<String> = contents.split('\n').map(|a| String::from(a)).collect();

    let mut sum: usize = 0;

    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        
        // Forward
        for i in 0..chars.len() {
            if chars[i].is_digit(10) {
                sum += 10*chars[i].to_digit(10).unwrap_or(0) as usize;
                // println!("{:?}", chars[i]);
                break;
            }
        }

        // Backward
        for i in 0..chars.len() {
            if chars[chars.len()-i-1].is_digit(10) {
                sum += chars[chars.len()-i-1].to_digit(10).unwrap_or(0) as usize;
                // println!("{:?}", chars[chars.len()-i-1]);
                break;
            }
        }
    }

    println!("sum: {:?}", sum);


    // PART B

    let digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0"];
    let digit_mapping: HashMap<&str, usize> = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .cloned()
    .collect();

    let dgt = "";

    let contents = fs::read_to_string("src/bin/day1/input.txt")
        .expect("Should have been able to read the file");

    let lines: Vec<String> = contents.split('\n').map(|a| String::from(a)).collect();

   let sum: usize = contents.lines().flat_map(|line| {
        let first_digit = digits.iter().filter_map(|&d| line.find(d).map(|pos| (d, pos))).min_by_key(|&(_, pos)| pos);
        let last_digit = digits.iter().filter_map(|&d| line.rfind(d).map(|pos| (d, pos))).max_by_key(|&(_, pos)| pos);

        let fd = first_digit.and_then(|(d, _)| d.parse::<usize>().ok()).or_else(|| digit_mapping.get(&first_digit.map(|(d, _)| d).unwrap()).cloned()).unwrap_or(0);
        let ld = last_digit.and_then(|(d, _)| d.parse::<usize>().ok()).or_else(|| digit_mapping.get(&last_digit.map(|(d, _)| d).unwrap()).cloned()).unwrap_or(fd);

        Some(10 * fd + ld)
    }).sum();

    println!("sum: {:?}", sum);
}