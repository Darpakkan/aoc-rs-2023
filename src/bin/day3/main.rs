use std::env;
use std::fs;

fn main() {
    let contents = fs::read_to_string("src/bin/day3/input.txt")
        .expect("Should have been able to read the file");

    let lines: Vec<String> = contents.split('\n').map(|a| String::from(a)).collect();

    
    // Part A
    let mut sum = 0;

    let mut array: Vec<Vec<char>> = Vec::new();

    for line in lines {
        array.push(line.chars().collect());
    }

    let ch = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.'];
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    while i < array.len() as i32 {
        while j < array[0].len() as i32 {

            // println!("CHECKING: {i}, {j}");
            let mut start_idx = j;
            let mut end_idx = j;
            if array[i as usize][j as usize].is_digit(10) {
                let mut is_surrounded = false;
                for x in Vec::<i32>::from([-1, 0, 1]).iter() {
                    for y in Vec::<i32>::from([-1, 0, 1]).iter() {
                        if !(*x == 0 && *y == 0) {
                            if i + *x >= 0 && i + *x < array.len() as i32 && j + *y >= 0 && j + *y < array[0].len() as i32 {
                                if !ch.contains(&array[(i + *x) as usize][(j + *y) as usize]){
                                    
                                    is_surrounded = true;
                                    
                                    // Find the num
                                    let mut num = String::new();
                                    while start_idx > 0 && array[i as usize][start_idx as usize-1].is_digit(10) {
                                        start_idx-=1;
                                    }
                                    while end_idx < array[0].len() as i32 -1 && array[i as usize][end_idx as usize+1].is_digit(10) {
                                        end_idx+=1;
                                    }
                                    while(start_idx <= end_idx){
                                        num.push(array[i as usize][start_idx as usize]);
                                        start_idx += 1;
                                    }

                                    // println!("FOUND: {num}");
                                    // println!("i,j: {i},{j}");

                                    sum += num.parse::<usize>().unwrap();
                                }
                            }
                        }
                    }
                }

                if is_surrounded {
                    j = end_idx+1;
                }
            }

            j += 1;
        }
        i += 1;
        j = 0;
    }

    println!("sum: {}", sum);

    // Part B

    let contents = fs::read_to_string("src/bin/day3/input.txt")
        .expect("Should have been able to read the file");

    let lines: Vec<String> = contents.split('\n').map(|a| String::from(a)).collect();


    let mut array: Vec<Vec<char>> = Vec::new();
    for line in lines {
        array.push(line.chars().collect());
    }
    let mut sum = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    while i < array.len() as i32 {
        while j < array[0].len() as i32 {

            // println!("CHECKING: {i}, {j}");
            if array[i as usize][j as usize] == '*' {
                let mut surrounding_num_posns = Vec::<((usize, usize))>::new();
                let mut surrounding_num = Vec::<usize>::new();
                for x in Vec::<i32>::from([-1, 0, 1]).iter() {
                    for y in Vec::<i32>::from([-1, 0, 1]).iter() {
                        if !(*x == 0 && *y == 0) {
                            if i + *x >= 0 && i + *x < array.len() as i32 && j + *y >= 0 && j + *y < array[0].len() as i32 {
                                let mut start_idx = j+*y;
                                let mut end_idx = j+*y;
                                if array[(i + *x) as usize][(j + *y) as usize].is_digit(10){
                                    
                                    
                                    // Find the num
                                    let mut num = String::new();
                                    while start_idx > 0 && array[(i + *x) as usize][start_idx as usize-1].is_digit(10) {
                                        start_idx-=1;
                                    }
                                    while end_idx < array[0].len() as i32 -1 && array[(i+*x) as usize][end_idx as usize+1].is_digit(10) {
                                        end_idx+=1;
                                    }
                                    
                                    // So we don't count the same number twice
                                    if !surrounding_num_posns.contains(&((i+*x) as usize, start_idx as usize)){
                                        surrounding_num_posns.push(((i+*x) as usize, start_idx as usize));
                                        while start_idx <= end_idx {
                                            num.push(array[(i + *x) as usize][start_idx as usize]);
                                            start_idx += 1;
                                        }
                                        surrounding_num.push(num.parse::<usize>().unwrap());
                                    }


                                }
                            }
                        }
                    }
                }

                if surrounding_num.len() == 2 {
                    // Add the gear ratios to sum
                    // println!("FOUND: {:?}", surrounding_num);
                    sum += surrounding_num[0]*surrounding_num[1];
                }
            }

            j += 1;
        }
        i += 1;
        j = 0;
    }

    println!("sum: {}", sum);
}