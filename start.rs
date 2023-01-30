use std::{fs::File, io::{BufReader, self, BufRead}};

fn main() -> std::io::Result<()> {
    let mut arguments: Vec<String> = std::env::args().skip(1).collect();
    match arguments.len() {
        0 => {
            println!("Usage: start filenames");
            println!("Put '-#' after start for specific amoumnt of lines");
        }
        _ => {
            let mut n = 10;
            let c = &arguments[0][0..1];
            if c == "-" {
                n = arguments[0][1..].parse().unwrap();
                arguments.remove(0);
            } 
            for arg in arguments {
                println!("File:  {arg}");
                print_lines(arg, n);
                println!("---------")
            }
        }
    }
    Ok(())
}


fn print_lines(filename: String, n: i32) {
    let lines = read_lines(filename);
    let mut count = 0;
    for line in lines {
        if count < n {
            println!("{}", line.unwrap());
            count += 1;
        }
        else {
            break;
        }
    }
}


fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    let file = File::open(filename).unwrap(); 
    return io::BufReader::new(file).lines(); 
}