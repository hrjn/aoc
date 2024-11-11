use std::fs::File;
use std::io::{BufReader, Read};

pub fn d01p01() -> std::io::Result<i32> {
    let input_fname = "inputs/2015/d01p1.txt";
    let input_file = File::open(input_fname)?;
    let mut input_reader = BufReader::new(input_file);
    let mut buffer = [0; 1];
    let mut counter = 0;
    while input_reader.read_exact(&mut buffer).is_ok() {
        let ch = buffer[0] as char;
        match ch {
            '(' => counter += 1,
            ')'=> counter -= 1,
            _ => {}
        }
    };
    Ok(counter)
}