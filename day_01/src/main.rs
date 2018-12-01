use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    let prog_name = args.next();
    if let Some(file_name) = args.next() {
        let mut first_time = true;
        let mut freq : i32 = 0;
        let mut seen : HashSet<i32> = HashSet::new();
        let file = File::open(&file_name)?;
        let input = BufReader::new(file);
        let lines : Vec<i32> = input.lines().map(|x| { x.unwrap().parse::<i32>().unwrap() }).collect();
        
        loop {
            
            for line in &lines {
                freq += line;
                if seen.contains(&freq) {
                    println!("Part 2: {}", freq);
                    return Ok(());
                }
                seen.insert(freq);
            }
            if first_time {
                println!("Part 1: {}", freq.to_string());
                first_time = false;
            }
        }
    }
    else {
        println!("Usage: {} <input_file>", prog_name.unwrap());
    }
    Ok(())
}
