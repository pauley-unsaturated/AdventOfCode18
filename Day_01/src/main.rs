use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let input = io::stdin();
    let mut freq = 0;
    for line_opt in input.lock().lines() {
        let line = line_opt?;
        freq += line.parse::<i32>().unwrap();
    }
    println!("{}", freq.to_string());
    
    Ok(())
}
