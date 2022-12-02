use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut cals = Vec::new();

    if let Ok(lines) = read_lines("../input") {
        let mut total: i32 = 0;
        for line in lines {
            
            if let Ok(cal) = line {
                if cal == "" {
                    cals.push(total);
                    total=0;
                }
                else {
                    total = total+cal.parse::<i32>().unwrap();
                }
            }
        }
    }
    let mut max: i32 = 0;
    for cal in cals {
        if cal > max {
            max = cal;
        }
    }
    println!("highest calories: {}", max);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
