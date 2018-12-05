extern crate regex;
extern crate chrono;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::io::BufReader;

use regex::Regex;

use chrono::{NaiveDateTime, Duration, Timelike};

struct Guard {
    sleeping_from : Option<NaiveDateTime>,
    total_slept_min : i32,
    slept_by_minute : HashMap<u32,i32>
}

impl Guard {
    fn new() -> Guard {
        Guard {
            sleeping_from: None,
            total_slept_min: 0,
            slept_by_minute: HashMap::new()
        }
    }
}

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    let prog_name = args.next();

    if let Some(file_name) = args.next() {
        let file = File::open(&file_name)?;
        let input = BufReader::new(file);
        let mut lines : Vec<String> = input.lines().map(|x|{x.unwrap()}).collect();

        let date_fmt = "%F %H:%M";
        let re = Regex::new(r"\[([^\]]+)\] (.*)").unwrap();
        let begin_re : Regex = Regex::new(r"Guard #(\d+) begins shift").unwrap();
        let sleep_re : Regex = Regex::new(r"falls asleep").unwrap();
        let wakes_re : Regex = Regex::new(r"wakes up").unwrap();

        lines.sort_by(|a, b| {
            let date_str_a = &re.captures(a).unwrap()[1];
            let date_str_b = &re.captures(b).unwrap()[1];
            //println!("a=[{}] b=[{}]", date_str_a, date_str_b);
            let date_a = NaiveDateTime::parse_from_str(date_str_a, date_fmt).unwrap();
            let date_b = NaiveDateTime::parse_from_str(date_str_b, date_fmt).unwrap();
            date_a.cmp(&date_b)
        });

        let mut guards = HashMap::<i32,Guard>::new();
        let mut cur_guard : Option<i32> = None;
        
        for line in &lines {
            if let Some(captures) = re.captures(&line) {
                use std::collections::hash_map::Entry;

                let timestamp = &NaiveDateTime::parse_from_str(&captures[1], date_fmt).unwrap();
                let event = &captures[2];
                //println!("timestamp=[{}] event= {}", timestamp, event);
                
                if let Some(capture) = begin_re.captures(event) {
                    let guard_num = capture[1].parse::<i32>().unwrap();
                    cur_guard = Some(guard_num);
                    // add a new guard if we haven't seen this one before
                    guards.entry(guard_num).or_insert(Guard::new());
                    //println!("Switched to guard {}", guard_num);
                }
                else if let Some(ref guard_num) = cur_guard {
                    if let Some(g) = guards.get_mut(guard_num) { 
                        if sleep_re.is_match(event) {
                            g.sleeping_from = Some(*timestamp);
                            //println!("Guard sleeping at {}", timestamp);
                        }
                        else if wakes_re.is_match(event) {
                            let minutes = (*timestamp - g.sleeping_from.unwrap()).num_minutes();
                            g.total_slept_min += minutes as i32;
                            let mut iter_date = g.sleeping_from.unwrap();
                            g.sleeping_from = None;
                            //println!("Guard wakes at {} (slept {} minutes)", timestamp, minutes);
                            while iter_date != *timestamp {
                                g.slept_by_minute.entry(iter_date.hour() * 60 + iter_date.minute()).and_modify(|x| {*x += 1}).or_insert(1);
                                iter_date += Duration::minutes(1);
                            }
                        }
                    }
                }
            }
            else {
                println!("No match: {}", line);
            }
        }
        println!("**********");

        // PART 1
        let dummy = Guard::new();                  
        let (id, guard) = guards.iter().fold((0, &dummy), |(id_max, guard_max), (&id, guard)| {
            if guard_max.total_slept_min < guard.total_slept_min {
                (id, guard)
            }
            else {
                (id_max, &guard_max)
            }
        });

        println!("Guard {} slept for {} minutes", id, guard.total_slept_min);
        
        let (max_minute, max_count) = guard.slept_by_minute.iter().fold((0, 0), |(max_minute, max_count), (&minute, &count)| {
            if max_count < count {
                (minute, count)
            }
            else {
                (max_minute, max_count)
            }
        });
        println!("Guard {} slept the most at minute {} (count={})", id, max_minute, max_count);
        println!("Part 1: {}", id as u32 * max_minute);

        // PART 2
        let (id, minute, count) = guards.iter().fold((0, 0, 0), |(id_max, minute_max, count_max), (&id, guard)| {
            let (minute, count) = guard.slept_by_minute.iter().fold((0, 0), |(max_minute, max_count), (&minute, &count)| {
                if max_count < count {
                    (minute, count)
                }
                else {
                    (max_minute, max_count)
                }
            });
            if count_max < count {
                (id, minute, count)
            }
            else {
                (id_max, minute_max, count_max)
            }
        });
        println!("Guard {} had the highest count of a minute slept at {} (count={})",
                 id, minute, count);
        println!("Part 2: {}", id as u32 * minute);
    }
    else {
        println!("Usage: {} <input_file>", prog_name.unwrap());
    }
    Ok(())
}
