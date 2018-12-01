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

        loop {
            let mut f = File::open(&file_name)?;
            let input = BufReader::new(f);
            
            for line_opt in input.lines() {
                let line = line_opt?;
                freq += line.parse::<i32>().unwrap();
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
