use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut cals = Vec::new();

    if let Ok(lines) = read_lines("./inputs/1") {
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
    let mut top_3 = vec![cals[0], cals[1], cals[2]];
    top_3.sort();

    for cal in cals {
        if cal > top_3[0] {
            top_3.remove(0);
            top_3.push(cal);
            top_3.sort();
        }
    }
    let sum = top_3[0] + top_3[1] + top_3[2];
    println!("[{}, {}, {}]", top_3[0], top_3[1], top_3[2]);
    println!("sum: {}", sum);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
