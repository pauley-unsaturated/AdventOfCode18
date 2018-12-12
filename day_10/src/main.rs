extern crate regex;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashSet;
use std::time::Duration;
use std::thread;

use regex::Regex;


struct Vec2 { x: i32, y: i32 }

struct Particle { position: Vec2, velocity: Vec2 }

impl Particle {
    fn step(&mut self, n: i32) -> () {
        self.position.x += self.velocity.x * n;
        self.position.y += self.velocity.y * n;
    }
}

// Prints from min,min to max,max
fn print_particles(particles: &Vec<Particle>, min: &Vec2, max: &Vec2) -> () {
    let mut set = HashSet::<(i32,i32)>::new();
    let xscale = 120;
    let yscale = 20;
    
    
    for p in particles {
        let x_interp = (p.position.x - min.x) as f32 / (max.x - min.x) as f32;
        let x = (x_interp * xscale as f32) as i32;
        let y_interp = (p.position.y - min.y) as f32 / (max.y - min.y) as f32;
        let y = (y_interp * yscale as f32) as i32;
        
        set.insert((x, y));
    }
    for y in 0..=yscale {
        for x in 0..=xscale {
            let output = if set.contains(&(x,y)) { "*" } else { " " };
            print!("{}", output);
        }
        println!("");
    }
    println!("");
}

fn find_bounds(particles: &Vec<Particle>) -> (Vec2,Vec2) {
    let mut min = Vec2{x: std::i32::MAX, y: std::i32::MAX};
    let mut max = Vec2{x: std::i32::MIN, y: std::i32::MIN};

    for p in particles {
        min.x = std::cmp::min(min.x, p.position.x);
        min.y = std::cmp::min(min.y, p.position.y);
        max.x = std::cmp::max(max.x, p.position.x);
        max.y = std::cmp::max(max.y, p.position.y);
    }
    (min, max)
}

fn move_particles(particles: &mut Vec<Particle>, num_steps: i32) -> () {
    for p in particles {
        p.step(num_steps);
    }
}

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    let prog_name = args.next();

    let mut particles : Vec<Particle> = Vec::new();
    let re = Regex::new(r"position=<\s*([^,]+),\s*([^>]+)> velocity=<\s*([^,]+),\s*([^>]+)>").unwrap();
    
    if let Some(file_name) = args.next() {
        let mut f = File::open(file_name)?;
        let mut reader = BufReader::new(f);
        
        for line in reader.lines() {
            let line = line?;
            if let Some(captures) = re.captures(&line) {
                let position = Vec2{ x: captures[1].parse::<i32>().unwrap(),
                                     y: captures[2].parse::<i32>().unwrap() };
                let velocity = Vec2{ x: captures[3].parse::<i32>().unwrap(),
                                     y: captures[4].parse::<i32>().unwrap() };
                particles.push(Particle{position: position, velocity: velocity});
            }
            else {
                println!("No match: {}", line);
            }            
        }
        for p in particles.iter() {
            println!("{}, {}    {}, {}", p.position.x, p.position.y,
                     p.velocity.x, p.velocity.y);
        }
        
        let (min, max) = find_bounds(&particles);
        println!("initial bounds: ({}, {})", max.x - min.x, max.y - min.y);
        if max.x - min.x <= 250 {
            print_particles(&particles, &min, &max);
        }

        let mut num_steps = 0;
        let mut stepsize = 100;
        loop {
            let (min,max) = find_bounds(&particles);
            if max.x - min.x <= 200 {
                stepsize = 1;
            }
            println!("{}: ({}, {})", num_steps, max.x - min.x, max.y - min.y);
            move_particles(&mut particles, stepsize);
            num_steps += stepsize;
            print_particles(&particles,&min,&max);
            if stepsize == 1 {
                thread::sleep(Duration::from_millis(1000))
            }
        }
    }
    else {
        println!("Usage: {} <input_file>", prog_name.unwrap());
    }
    Ok(()) 
}
