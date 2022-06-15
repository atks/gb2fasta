use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("printing {:?}", args);
    let gb_file: String = args[1].clone();
    println!("{:?}", gb_file);

    if gb_file != "-" {
        if let Ok(lines) = read_lines(gb_file) {
            for line in lines {
                if let Ok(ip) = line {
                    println!("{}", ip)
                }
            }
        }
    }
    else {
        println!("reading from STDIN");

        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            println!("{}", line.unwrap());
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}