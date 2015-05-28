use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn sum_file(path: &str) -> i64 {
    let mut sum : i64 = 0;
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("couldn't open {}: {}", path, e),
    };
    let reader = BufReader::new(file);

    for readline in reader.lines() {
        let line = match readline {
            Ok(readline) => readline,
            Err(e) => panic!("couldn't read from {}: {}", path, e),
        };
        match line.trim().parse::<i64>() {
            Ok(v) => sum += v,
            Err(_) => panic!("invalid integer in {}: {}", path, line),
        }
    }
    sum
}

fn main() {
    println!("Sum: {}", sum_file("numbers.txt"));
}
