use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    let fname = "./inputs/3";
    let file = File::open(fname).unwrap();
    let reader = BufReader::new(file);

    let mut sum: u32 = 0;
    let mut curr: Vec<String> = Vec::new(); 
    for (_index, line) in reader.lines().enumerate() {
        curr.push(line.unwrap());
        if curr.len() == 3 {
            for i in curr[0].chars() {
                if curr[1].contains(i) && curr[2].contains(i) {
                    if i as u32 > 96 {
                        sum = sum + i as u32 - 96;    
                    }
                    else {
                        sum = sum + i as u32 - 38;
                    }
                    break;
                }
            }
            curr.clear();
        }
    }
    println!("Total sum is: {}", sum);
}
