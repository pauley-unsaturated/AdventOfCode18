extern crate regex;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;
use std::collections::HashMap;
use std::io::BufReader;

use regex::Regex;

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    let prog_name = args.next();

    if let Some(file_name) = args.next() {
        let file = File::open(&file_name)?;
        let input = BufReader::new(file);
        let lines : Vec<String> = input.lines().map(|x|{x.unwrap()}).collect();
        let re = Regex::new(r"#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)").unwrap();
        let mut taken = HashMap::new();
        let mut dup = HashSet::new();

        let mut clean = HashSet::new();
        for line in &lines {            
            //println!("Reading line: {}", &line);
            if let Some(captures) = re.captures(&line) {
                let coords : Vec<i32> = captures.iter().skip(1).map(|x|{ x.unwrap().as_str().parse::<i32>().unwrap() }).collect();
                let (id, x_0, y_0, w, h) = (coords[0], coords[1], coords[2], coords[3], coords[4]);
                //println!("Parsed as {},{} {}x{}", x_0, y_0, w, h);
                clean.insert(id);
                for x in x_0..(x_0 + w) {
                    for y in y_0..(y_0 + h) {
                        let point = (x, y);
                        //println!("({},{})", x,y);
                        if taken.contains_key(&point) {
                            dup.insert(point);
                            clean.remove(&id);
                            clean.remove(&taken[&point]);
                        }
                        else {
                            taken.insert(point, id);
                        }
                    }
                }                
            }
            else {
                println!("Bad input \"{}\" didn't match!", &line);
            }
        }
        println!("Part1: {}", dup.len());
        for id in clean {
            println!("Part2: {}", id);
        }
    }
    else {
        println!("Usage: {} <input_file>", prog_name.unwrap());
    }
    Ok(())
}
