use std::env;
use std::fs;
use std::collections::HashMap;
use std::u128::MAX;

fn main() {
    let contents = fs::read_to_string("src/bin/day4/input.txt")
        .expect("Should have been able to read the file");

    let lines: Vec<String> = contents.split('\n').map(|a| String::from(a)).collect();

    
    // Part A
    let mut sum = 0;
    
    for line in lines.clone() {
        let parts: Vec<String> = line.split('|').map(|a| a.to_string()).collect();

        let (partA, partB) = (&parts[0], &parts[1]);
        let numsA: Vec<String> = partA.split(':').map(|a| a.to_string()).collect::<Vec<String>>()[1].split_ascii_whitespace().map(|a| String::from(a)).collect();
        let numsB: Vec<String> = partB.split_ascii_whitespace().map(|a| String::from(a)).collect();

        let card_nums: Vec<usize> = numsA.iter().map(|a| a.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let found_nums: Vec<usize> = numsB.iter().map(|a| a.parse::<usize>().unwrap()).collect::<Vec<usize>>();

        let mut score = 1;

        // println!("FOUND NUMS: {:#?}", found_nums);
        // println!("CARD NUMS: {:#?}", card_nums);

        for num in found_nums {
            if card_nums.contains(&num) {
                score *= 2;
            }
        }

        sum += score/2;
    }

    println!("sum: {}", sum);

    // part B

    let mut card_db: HashMap<usize, usize> = HashMap::new();

    let MAX_CARD: usize = 214;

    let mut sum = 0;
    
    let mut idx = 1;

    for line in lines {
        let parts: Vec<String> = line.split('|').map(|a| a.to_string()).collect();

        let (partA, partB) = (&parts[0], &parts[1]);
        let numsA: Vec<String> = partA.split(':').map(|a| a.to_string()).collect::<Vec<String>>()[1].split_ascii_whitespace().map(|a| String::from(a)).collect();
        let numsB: Vec<String> = partB.split_ascii_whitespace().map(|a| String::from(a)).collect();

        let card_nums: Vec<usize> = numsA.iter().map(|a| a.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let found_nums: Vec<usize> = numsB.iter().map(|a| a.parse::<usize>().unwrap()).collect::<Vec<usize>>();

        let mut no_matching = 0;
        
        for num in found_nums {
            if card_nums.contains(&num) {
                no_matching += 1;
            }
        }

        
        
        // Add current card to db
        let key = idx;
        let entry = card_db.entry(key).or_insert(0);
        
        // Increment the value
        *entry += 1;

        let mut a = *card_db.get(&idx).unwrap_or(&1);

        while a > 0 {
            let mut i = no_matching;
            while i > 0 {
                // Add Card to db
                if idx + i <= MAX_CARD {

                    let key = idx + i;
                    let entry = card_db.entry(key).or_insert(0);
                    
                    // Increment the value
                    *entry += 1;
                } 
                i -= 1;
            }
            a -= 1;
        }
        
        // println!("CURRENT CARD: {:#?}", idx);
        // println!("NUM MATCHING: {:#?}", no_matching);
        // println!("CARD DB: {:#?}", card_db);

        idx += 1;
    }

    
    println!("res: {}", card_db.values().sum::<usize>());
}