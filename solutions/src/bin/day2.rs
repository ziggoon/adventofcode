use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./inputs/2") {
        // from @c4r3t was too lazy to write my own match <3
        let mut score: i32 = 0;
        for line in lines {
            let line = line.unwrap(); // Ignore errors.
            if line == "" { // if the line ended
            } else {
                let test:Vec<_> = line.split(" ").collect();
                if test.len() != 0 {
                    match (test[0],test[1]) {
                        ("A","X") => score=score+1+3,
                        ("A","Y") => score=score+2+6,
                        ("A","Z") => score=score+3+0,
                        ("B","X") => score=score+1+0,
                        ("B","Y") => score=score+2+3,
                        ("B","Z") => score=score+3+6,
                        ("C","X") => score=score+1+6,
                        ("C","Y") => score=score+2+0,
                        ("C","Z") => score=score+3+3,
                        (_,_) => (),
                    }
                } else {
                    println!("ERROR");
                }
            }
        }
        println!("Total score is : {}",score);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
