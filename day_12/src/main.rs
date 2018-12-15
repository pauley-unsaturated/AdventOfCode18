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
struct State(Vec<Pot>);

impl State {
    fn process(&self, rules: &HashSet<State>) -> State {
        let State(v) = self;
        let mut next : Vec<Pot> = Vec::new();
        let beg_filler = vec![Pot::Empty; 5];
        let end_filler = vec![Pot::Empty; 5];
        let v : Vec<Pot> = beg_filler.iter().chain(v.iter()).chain(end_filler.iter()).map(|x| {x.clone()}).collect();
        
        for window in v.windows(5) {
            let s = State(window.to_vec());
            next.push( if rules.contains(&s) { Pot::Plant } else { Pot::Empty } );
        }
        State(next)
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
        const STEPS : usize = 20;
        let mut cur_state = init_state;
        for n in 0..STEPS {            
            println!("{:02}: {}", n, cur_state);
            cur_state = cur_state.process(&rule_set);
        }
        println!("{:02}: {}", STEPS, cur_state);
        // at this point, state starts at -3 * STEPS
        let State(v) = cur_state;
        let iter = v.into_iter();
        let mut sum = 0;
        for (idx, pot) in (0..).zip(iter) {
            if pot == Pot::Plant {
                let val = idx as i32 - (3 * STEPS) as i32;
                print!("{} ", val);
                sum += val;
            }
        }
        println!("");
        println!("Part 1: {}", sum);
    }
    else {
        println!("Usage: {} <input_file>", prog_name.unwrap());
    }
    Ok(())
}

