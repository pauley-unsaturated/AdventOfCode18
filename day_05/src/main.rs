use std::io;
use std::io::prelude::*;
use std::fs::File;

fn collapse (input: &Vec<char>) -> Vec<char> {
    let mut cur : Vec<char> = input.to_vec();

    loop {
        // Cancel the polar opposites
        let next = cur.iter().fold(Vec::<char>::new(),|mut l, &cur| {
            if let Some(prev) = l.pop() {
                if prev != cur && prev.to_uppercase().next() == cur.to_uppercase().next() {
                    return l;
                }
                l.push(prev);
                l.push(cur);
            }
            else {
                l.push(cur);
            }
            l
        });

        if next.len() != cur.len() {
            cur = next;
        }
        else {
            // If we didn't cancel any, we're done
            return cur;
        }
    }
}

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    let prog_name = args.next();

    if let Some(file_name) = args.next() {
        let mut file = File::open(&file_name)?;
        let mut buf = Vec::<u8>::new();
        file.read_to_end(&mut buf)?;

        let mut buf : Vec<char> = buf.into_iter().map(|x|{x as char}).collect();

        //let s: String = buf.iter().collect();
        //println!("Input = {} ({})", s, s.len());

        let part_1 = collapse(&buf);
        println!("Part 1: {}", part_1.len());

        static ASCII_LOWER: [char;26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
                                         'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
                                         's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
        let smallest = ASCII_LOWER.iter().fold(None as Option<Vec<char>>, |smallest, &c| {
            let filtered = buf.iter().filter_map(|&x| {
                if x == c || x == c.to_uppercase().next().unwrap() {
                    None
                }
                else {
                    Some(x)
                }
            }).collect();
            let cur = collapse(&filtered);
            if let Some(ref s) = smallest {
                if cur.len() < s.len() {
                    Some(cur)
                }
                else {
                    Some(s.to_vec())
                }
            }
            else {
                Some(cur)
            }
        });
        
        println!("Part 2: {}", smallest.unwrap().len());

        
    }
    else {
        println!("Usage: {} <input_file>", prog_name.unwrap());
    }
    Ok(())        
}
