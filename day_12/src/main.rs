extern crate regex;


use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashSet;
use std::fmt;

use regex::Regex;

#[derive(Debug)]
#[derive(Hash)]
#[derive(PartialEq, Eq)]
#[derive(Clone)]
enum Pot {
    Empty,
    Plant
}

impl From<char> for Pot {
    fn from(c: char) -> Pot {
        match c {
            '.' => Pot::Empty,
            '#' => Pot::Plant,
            _   => { panic!("{} is not a Pot", c); }
        }
    }
}           
    

impl fmt::Display for Pot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = match self {
            Pot::Empty => '.',
            Pot::Plant => '#'
        };
        write!(f, "{}", c)        
    }
}

#[derive(Hash)]
#[derive(Eq)]
#[derive(Clone)]
struct State(Vec<Pot>);

impl State {
    fn process(&self, rules: &HashSet<State>) -> State {
        let State(v) = self;
        let mut next : Vec<Pot> = Vec::new();
        let beg_filler = vec![Pot::Empty; 2];
        let end_filler = vec![Pot::Empty; 3];
        let v : Vec<Pot> = beg_filler.iter().chain(v.iter()).chain(end_filler.iter()).map(|x| {x.clone()}).collect();
        
        for window in v.windows(5) {
            let s = State(window.to_vec());
            next.push( if rules.contains(&s) { Pot::Plant } else { Pot::Empty } );
        }
        State(next)
    }

    fn value(&self, offset: i32) -> i32 {
        let State(v) = self;
        let iter = v.iter();
        let mut sum = 0;
        for (idx, pot) in (0..).zip(iter) {
            if *pot == Pot::Plant {
                let val = idx as i32 - offset as i32;
                sum += val;
            }
        }
        sum
    }
}

impl<'a> From<&'a str> for State {
    fn from(s: &'a str) -> State {
        let mut state : Vec<Pot> = Vec::new();
        for c in s.chars() {
            state.push( match c {
                '#' => Pot::Plant,
                '.' => Pot::Empty,
                _  => { panic!("'{}' is Neither Pot nor empty!", c ); }
            });
        }
        State(state)
    }
}

impl std::cmp::PartialEq for State {
    fn eq(&self, other: &State) -> bool {
        let (State(v), State(o)) = (self, other);
        v.iter().eq(o.iter())
    }
}

/*
impl std::hash::Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let State(v) = self;
        v.iter().map(|x| { x.hash(state); });
    }
}
*/
        
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let State(v) = self;
        for pot in v.iter() {
            pot.fmt(f)?;
        }
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    let prog_name = args.next();
    
    if let Some(file_name) = args.next() {
        let mut f = File::open(file_name)?;
        let mut reader = BufReader::new(f);
        
        let init_re = Regex::new(r"initial state: ([#.]+)").unwrap();
        let rule_re = Regex::new(r"([#.]{5}) => ([#.])").unwrap();

        let mut lines = reader.lines().into_iter();

        // Parse the initial line
        let init_line = lines.next().unwrap()?;

        let init_state = if let Some(captures) = init_re.captures(&init_line) {
            State::from(&captures[1])
        }
        else {
            panic!("Invalid initial state format: {}", init_line);
        };
        println!("Initial state: {}", init_state);

        // Skip a line...
        lines.next();

        let mut rule_set : HashSet<State> = HashSet::new();
        
        for l in lines {
            let line = l?;
            if let Some(captures) = rule_re.captures(&line) {
                let rule = State::from(&captures[1]);
                let value = Pot::from(captures[2].chars().next().unwrap());
                println!("{} => {}", rule, value);
                if value == Pot::Plant {
                    rule_set.insert(rule);
                }
            }
            else {
                panic!("Invalid rule format: {}", line);
            }
        }

        // process        
        const PART_1_STEPS : usize = 20;
        let mut history = Vec::<i32>::new();
        let mut cur_state = init_state.clone();
        let mut step = 0;
        for n in 0..PART_1_STEPS {
            println!("{}: {}", n, cur_state);
            cur_state = cur_state.process(&rule_set);
            step += 1;
        }
        println!("{:02}: {}", PART_1_STEPS, cur_state);
        // at this point, state starts at -3 * STEPS
        let part_1 = cur_state.value(3);
        println!("Part 1: {}\n", part_1);

        // Part 2 can't be brute forced. It's way way too many steps.
        // We need to detect when we are just shifting to the right and
        // determine how many more shifts we'll perform        
        history.push(part_1);
        let mut delta;
        loop {
            cur_state = cur_state.process(&rule_set);
            step += 1;
            history.push(cur_state.value(3));
            while history.len() > 3 {
                history.remove(0);
            }
            if history.len() == 3 {
                if history[1] - history[0] == history[2] - history[1] {
                    delta = history[2] - history[1];
                    break;
                }
            }
        }
        let loop_init_value = cur_state.value(3) as i64;
        println!("Found a loop at step {}, value = {}, delta = {}",
                 step, loop_init_value, delta);
        const PART_2_STEPS : i64 = 50000000000;
        let steps_left = PART_2_STEPS - step;
        let final_value = steps_left * delta as i64 + loop_init_value;
        println!("Part 2: {}", final_value);
    }
    else {
        println!("Usage: {} <input_file>", prog_name.unwrap());
    }
    Ok(())
}

