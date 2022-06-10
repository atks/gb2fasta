extern crate gb_io;

use std::env;
use std::fs::File;
use std::io;
use gb_io::reader::SeqReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("printing {:?}", args);
    let gb_file: String = args[1].clone();

    println!("{:?}", gb_file);

    let file = File::open(gb_file).unwrap();
    let stdout = io::stdout();
    for seq in SeqReader::new(file) {
        let seq = seq.unwrap();
        let rc = seq.revcomp();
        rc.write(stdout.lock()).unwrap();

        break;
    }



}
