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

    let mut sum: usize = 0;

    for line in lines {

        let mut first_digit: &str = "";
        let mut last_digit: &str = "";

        // Forward
        let mut first_position: usize = 999;
        for digit in digits {
            let pos = line.find(digit).unwrap_or(first_position);
            if pos < first_position {
                first_position = pos;
                first_digit = digit;
            }
        }

        // Reverse
        let mut last_position: usize = 0;
        for digit in digits {
            let pos = line.rfind(digit).unwrap_or(last_position);
            if pos > last_position {
                last_position = pos;
                last_digit = digit;
            }
        }

        let mut fd = 0;
        let mut ld = 0;

        if first_digit.len() == 1 {
            fd = first_digit.chars().nth(0).unwrap().to_digit(10).unwrap() as usize;
        } else {
            fd = *digit_mapping.get(first_digit).unwrap();
        }

        sum += 10*fd;

        if last_digit.len() == 1 {
            ld = last_digit.chars().nth(0).unwrap().to_digit(10).unwrap() as usize;
        } else {
            ld = *digit_mapping.get(last_digit).unwrap_or(&fd);
        }

        sum += ld;
        
    }

    println!("sum: {:?}", sum);
}