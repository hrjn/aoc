use std::fs::File;
use std::io::{BufReader, Read};

pub fn d01() -> std::io::Result<()> {
    let input_fname = "inputs/2015/d01p1.txt";
    let input_file = File::open(input_fname)?;
    let mut input_reader = BufReader::new(input_file);
    let mut buffer = [0; 1];
    let mut p1_out = 0;
    let mut p2_out = None;
    let mut idx = 0;
    while input_reader.read_exact(&mut buffer).is_ok() {
        let ch = buffer[0] as char;
        match ch {
            '(' => p1_out += 1,
            ')'=> p1_out -= 1,
            _ => {}
        }
        if p1_out < 0 && p2_out.is_none(){
            p2_out = Some(idx+1);
        }
        idx +=1;
    };
    println!("p1: {}, p2: {}", p1_out, p2_out.unwrap_or(0));
    Ok(())
}
