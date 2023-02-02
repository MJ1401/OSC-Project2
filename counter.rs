use std::{io::{BufRead, BufReader}, fs::File};

fn main() -> std::io::Result<()> {
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    match arguments.len() {
        0 => {
            println!("Usage: counter filenames");
            println!("Put '-w', '-l', or '-c' after counter for number of words, lines, or characters to display.");
        }
        _ => {
            if &arguments[0][0..1] == "-" {
                let mut part = arguments.clone();
                part.remove(0);
                for arg in part {
                    println!("File {arg}");
                    let arg1 = arg.clone();
                    let arg2 = arg.clone();
                    if arguments[0].contains("w") {
                        let buffer = get_buffer(arg);
                        let n = find_words(buffer);
                        println!("Words: {n}")
                    }
                    if arguments[0].contains("l") {
                        let buffer = get_buffer(arg1);
                        let n = find_lines(buffer);
                        println!("Lines: {n}")

                    }
                    if arguments[0].contains("c") {
                        let buffer = get_buffer(arg2);
                        let n = find_characters(buffer);
                        println!("Characters: {n}")
                    }
                }
                
            } else {
                for arg in arguments {
                    let arg1 = arg.clone();
                    let arg2 = arg.clone();
                    let arg3 = arg.clone();
                    let buffer = get_buffer(arg);
                    let buffer1 = get_buffer(arg1);
                    let buffer2 = get_buffer(arg2);
                    println!("File: {arg3}");
                    println!("Words: {}", find_words(buffer));
                    println!("Lines: {}", find_lines(buffer1));
                    println!("Characters: {}", find_characters(buffer2));
            }
            }
        }
    } 
    Ok(())
}


fn get_buffer(filename: String) -> BufReader<File> {
    let f = File::open(filename).unwrap();
    let buffer = BufReader::new(f);
    buffer
}



fn find_words(buffer: BufReader<File>) -> i32 {
    let mut count = 0;
    for line in buffer.lines() {
        let line = line.unwrap();
        for _ in line.split_whitespace() {
            count += 1
        }
    }
    count
}


fn find_lines(buffer: BufReader<File>) -> i32 {
    let mut count = 0;
    for _ in buffer.lines() {
        count += 1
    }
    count
}


fn find_characters(buffer: BufReader<File>) -> i32 {
    let mut count = 0;
    for line in buffer.lines() {
        let line = line.unwrap();
        count += line.len() as i32 + 1;
    }
    count
}
