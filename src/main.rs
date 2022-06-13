use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("printing {:?}", args);
    let gb_file: String = args[1].clone();

    println!("{:?}", gb_file);

    if let Ok(lines) = read_lines(gb_file) {
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip)
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// fn test() {
//     let args: Vec<String> = env::args().collect();
//     println!("printing {:?}", args);
//     let gb_file: String = args[1].clone();

//     println!("{:?}", gb_file);

//     let file = File::open(gb_file).unwrap();
//     let _stdout = io::stdout();
//     for seq in SeqReader::new(file) {
//         let seq = seq.unwrap();
//         // let rc = seq.revcomp();
//         // rc.write(stdout.lock()).unwrap();
//         println!("{}", seq.len.unwrap());

//         for n in 1..seq.len.unwrap() {
//             println!("{}", seq.seq[n]);
//         }

//         println!("LENGTH {}", seq.len.unwrap());

//         break;
//     }
// }