use std::io;

mod aoc15;

fn main() -> io::Result<()>{
    let d01p01 = aoc15::d01p01().unwrap();
    println!{"{}", d01p01};
    Ok(())
}
