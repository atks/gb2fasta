use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
}


// use std::{env, io, fs};
// use std::io::prelude::*;

// enum Input {
//     Standard(io::Stdin),
//     File(fs::File)
// }

// impl Input {

//     fn stdin() -> Input {
//         Input::Standard(io::stdin())
//     }

//     fn file(path: String) -> io::Result<Input> {
//         Ok(Input::File(try!(fs::File::open(path))))
//     }

//     fn from_arg(arg: Option<String>) -> io::Result<Input> {
//         Ok(match arg {
//             None       => Input::stdin(),
//             Some(path) => try!(Input::file(path))
//         })
//     }
// }

// impl io::Read for Input {

//     fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
//         match *self {
//             Input::Standard(ref mut s) => s.read(buf),
//             Input::File(ref mut f)     => f.read(buf),
//         }
//     }

// }

// fn load_inputs() -> io::Result<(Input, Input)> {
//     let mut args = env::args().skip(1);
//     Ok((try!(Input::from_arg(args.next())), try!(Input::from_arg(args.next()))))
// }

// fn main() {

//     let (mut input1, mut input2) = match load_inputs() {
//         Ok(inputs) => inputs,
//         Err(error) => panic!("Failed: {}", error),
//     };

//     let mut buf1 = [0u8; 8];
//     let mut buf2 = [0u8; 8];

//     let _ = input1.read(&mut buf1);
//     let _ = input2.read(&mut buf2);

//     println!("buf1 = {:?}, buf2 = {:?}", buf1, buf2);
// }


// use std::io::{self, BufRead};
// use std::env;
// use std::fs::File;
// use std::path::Path;

// fn main() -> io::Result<()> {
//     let stdin = io::stdin();
//     let mut lines = stdin.lock().lines();

//     while let Some(line) = lines.next() {
//         let length: i32 = line.unwrap().trim().parse().unwrap();

//         for _ in 0..length {
//             let line = lines
//                 .next()
//                 .expect("there was no next line")
//                 .expect("the line could not be read");

//             println!("{}", line);
//         }
//     }

//     Ok(())
// }

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     println!("printing {:?}", args);
//     let gb_file: String = args[1].clone();
//     println!("{:?}", gb_file);

//     if gb_file != "-" {
//         if let Ok(lines) = read_lines(gb_file) {
//             for line in lines {
//                 if let Ok(ip) = line {
//                     println!("{}", ip)
//                 }
//             }
//         }
//     }
//     else {
//         println!("reading from STDIN");

//         let buffer = io::BufReader::new(io::stdin());
//         let mut input_iter = buffer.lines();
//         input_iter.next();
//         input_iter.next();

//         for line in input_iter {
//             println!("{:?}", line);
//         }

//     }
// }

// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where P: AsRef<Path>, {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }