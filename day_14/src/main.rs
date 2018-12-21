extern crate regex;

use regex::Regex;

fn main() {
    let mut args = std::env::args();
    let prog_name = args.next();
    if let Some(input) = args.next().map(|x| { x.parse::<usize>().unwrap() }) {
        println!("Input: {}", input);

        let mut scores : Vec<usize> = vec![3, 7];
        let mut cur_scores : Vec<usize> = vec![0, 1];

        let mut scores_str = "37".to_string();
        
        let input_str = input.to_string();

        let re = Regex::new(&format!("^(.*){}", input_str)).unwrap();
        let mut part_2 : Option<usize> = None;
        let mut num_removed = 0;
        
        while scores.len() < input + 10 || part_2 == None {
            let mut new_scores : Vec<usize> = cur_scores.iter().fold(0, |sum, idx| { sum + scores[*idx] }).to_string().chars().map(|d| { d.to_digit(10).unwrap() as usize }).collect();
            if part_2 == None {
                scores_str.push_str(&new_scores.iter().fold(String::new(), |mut s, d| { s.push_str(&d.to_string()); s }));
            }
            scores.append(&mut new_scores);
            cur_scores = cur_scores.into_iter().map(|x| { (x + scores[x as usize] + 1) % scores.len() }).collect();

            if part_2 == None {
                if let Some(captures) = re.captures(&scores_str.clone()) {
                    part_2 = Some(captures[1].len() + num_removed);
                }
                else {
                    while scores_str.len() > input_str.len() {
                        scores_str.remove(0);
                        num_removed += 1;
                    }
                }
            }
        }

        print!("Part 1: ");
        for d in scores[input..(input + 10)].iter() {
            print!("{}", d.to_string());
        }
        println!("");

        let part_2 = part_2.unwrap();
        println!("Part 2: {}", part_2);
    }    
    else {
        println!("Usage: {} <input_file>", prog_name.unwrap());
    }
}
