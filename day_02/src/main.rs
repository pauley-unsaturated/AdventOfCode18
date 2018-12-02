use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashSet;
use std::io::BufReader;

fn n_duplicates(n :i32, string: &String) -> i32 {
    let mut table = [0; 256];
    let mut result : i32 = 0;
    for byte in string.as_bytes().iter() {
        table[*byte as usize] += 1;
    }
    for b in table.iter() {
        if *b as i32 == n {
            result += 1;
        }
    }
    result
}

fn diff_strs (a: &String, b: &String) -> String {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let min_len = std::cmp::min(a_bytes.len(), b_bytes.len());
    let max_len = std::cmp::max(a_bytes.len(), b_bytes.len());

    let mut common_chars = Vec::<char>::new();
    for i in 0..min_len {
        if a_bytes[i] == b_bytes[i] {
            common_chars.push(a_bytes[i] as char);
        }
    }
    common_chars.into_iter().collect()
}

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    let prog_name = args.next();

    if let Some(file_name) = args.next() {
        let file = File::open(&file_name)?;
        let input = BufReader::new(file);
        let lines : Vec<String> = input.lines().map(|x|{x.unwrap()}).collect();
        let mut check_2 = 0;
        let mut check_3 = 0;
        for line in &lines {
            if n_duplicates(2, &line) > 0 {
                check_2 += 1;
            }
            if n_duplicates(3, &line) > 0 {
                check_3 += 1;
            }
            for otherline in &lines {
                let remove_diffs = diff_strs(&line, &otherline);
                if remove_diffs.len() == line.len() - 1 {
                    println!("Part 2: {}", remove_diffs);
                }
            }
        }
        println!("Part 1: {}", check_2 * check_3);
    }
    else {        
        println!("Usage: {} <input_file>", prog_name.unwrap());
    }
    Ok(())
}
