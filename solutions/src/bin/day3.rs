use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let fname = "./inputs/3";
    let file = File::open(fname).unwrap();
    let reader = BufReader::new(file);

    let mut sum:u32 = 0;
    
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let length = line.len();
        //println!("length: {} length/2: {}", length, length/2);
        let line1 = &line[0..(length/2)];
        let line2 = &line[(length/2)..length];
        //println!("{} {}", line1, line2);
        let mut visited = Vec::new();
        for i in line1.chars() {
            for j in line2.chars() {
                if i == j {
                    if visited.contains(&i) {
                        break;
                    }
                    visited.push(i);
                    if i as u32 > 96 {
                        sum = sum + i as u32 - 96;
                    }
                    else {
                        sum = sum + i as u32 - 38;
                    }
                }
            }
        }
    }
    println!("Total sum is: {}", sum);
}
