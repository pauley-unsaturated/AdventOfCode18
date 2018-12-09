extern crate regex;

use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashSet;

use regex::Regex;


fn main() -> io::Result<()> {
    let mut args = std::env::args();
    let prog_name = args.next();

    if let Some(file_name) = args.next() {
        let mut file = File::open(&file_name)?;
        let mut reader = BufReader::new(file);
        let letters : Vec<char> = (b'A' ..= b'Z').map(|c| { c as char}).collect();

        let mut pending_letters : HashSet<char> = HashSet::new();
        let mut dependencies : Vec<HashSet<char>> = letters.iter().map(|_| { HashSet::<char>::new() }).collect();
        let mut reverse_deps : Vec<HashSet<char>> = dependencies.clone();
        let re = Regex::new(r"Step (.) must be finished before step (.) can begin.").unwrap();
        
        for line in reader.lines() {
            let line = line.unwrap();
            if let Some(captures) = re.captures(&line) {
                let required = captures[1].as_bytes()[0] as char;
                let step = captures[2].as_bytes()[0] as char;
                
                pending_letters.insert(step);
                pending_letters.insert(required);
                let step_num = step as usize - b'A' as usize;
                let req_num = required as usize - b'A' as usize;
                
                dependencies[step_num].insert(required);
                reverse_deps[req_num].insert(step);
            }
            else {
                println!("No match for line {}", &line);
            }         
        }

        let mut order = Vec::<char>::new();
        let mut total_time = 0;

        const NUM_WORKERS : usize = 5;
        const TIME_OFFSET : usize = 60;
        let mut workers : [Option<(char,usize)>; NUM_WORKERS] = [None; NUM_WORKERS]; // 2 workers for the test
        let mut waiting_letters : HashSet<char> = pending_letters.clone();
        
        while pending_letters.len() > 0 {
            let can_place = workers.iter().fold(false, |res, &x| { res || x == None });
            let mut did_place = false;
            if can_place {
                for &letter in letters.iter() {
                    let idx = letter as usize - b'A' as usize;
                    if waiting_letters.contains(&letter)
                        && dependencies[idx].len() == 0 {
                            for worker in &mut workers {
                                if None == *worker {
                                    let time = letter as usize - b'A' as usize + 1 + TIME_OFFSET;
                                    *worker = Some((letter, time));
                                    waiting_letters.remove(&letter);
                                    did_place = true;
                                    break;
                                }
                            }
                        }
                }
            }
            if !did_place {
                  // do work
                total_time += 1;
                for worker in &mut workers {
                    if let Some((l, time)) = *worker {
                        let time = time - 1;
                        if time == 0 {
                            pending_letters.remove(&l);
                            order.push(l);
                            for mut deps in dependencies.iter_mut() {
                                deps.remove(&l);
                            }
                            *worker = None
                        }
                        else {
                            *worker = Some((l, time));
                        }
                    }
                }
            }
        }
        println!("Part 1: {}", order.iter().collect::<String>());
        println!("Part 2: {}", total_time);
        
    }
    else {
        println!("Usage: {} <input_file>", prog_name.unwrap());
    }
    Ok(())        
}
