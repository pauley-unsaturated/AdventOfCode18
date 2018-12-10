#[macro_use]
extern crate intrusive_collections;
use intrusive_collections::{LinkedList, LinkedListLink};
use std::cell::Cell;

struct Node {
    link: LinkedListLink,
    value: Cell<u32>,
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.value == other.value
    }
}

intrusive_adapter!(NodeAdapter = Box<Node>: Node { link: LinkedListLink });

fn main() {
    let mut args = std::env::args();
    let prog_name = args.next();
    
    if let (Some(num_players), Some(max_value)) = (args.next(), args.next()) {
        let num_players = num_players.parse::<usize>().expect("first arg (num_players) must be an unsigned int");
        let max_value = max_value.parse::<u32>().expect("second arg (max_value) must be an unsigned int");
        let mut stones = LinkedList::new(NodeAdapter::new());

        /* Debug printing
        let printlist : *const LinkedList<NodeAdapter> = &stones;
         */
        
        let mut scores = vec![0; num_players];
        let mut cur_player = 0;

        let mut cursor = stones.cursor_mut();

        let b = Box::new(Node {
            link: LinkedListLink::new(),
            value: Cell::new(0),
        });
        cursor.insert_after(b);
        
        for value in 1..=max_value {
            if value % 23 != 0 {
                cursor.move_next();
                if cursor.is_null() { cursor.move_next() };
                let v = Box::new(Node {
                    link: LinkedListLink::new(),
                    value: Cell::new(value),
                });
                cursor.insert_after(v);
                cursor.move_next();
            }
            else {
                scores[cur_player] += value;
                for _ in 0..7 {
                    cursor.move_prev();
                    if cursor.is_null() { cursor.move_prev(); }                
                }
                scores[cur_player] += cursor.get().unwrap().value.get();
                cursor.remove();
                if cursor.is_null() { cursor.move_next(); }
            }
            cur_player = (cur_player + 1) % num_players;
            /* Debug Printing
            unsafe {                
                let mut print_cursor = (*printlist).cursor();
                print!("[{:02}] ", value);
                print_cursor.move_next();
                while !print_cursor.is_null() {
                    if print_cursor.get().unwrap() == cursor.get().unwrap() { print!("(") }
                    print!("{}", print_cursor.get().unwrap().value.get());
                    if print_cursor.get().unwrap() == cursor.get().unwrap() { print!(")") }
                    print!(" ");
                    print_cursor.move_next();
                }
                println!("");
            }
             */
        }
        let part_1 = scores.iter().skip(1).fold(scores[0], |max, &x| { std::cmp::max(max, x) });
        println!("Part 1: {}", part_1);
    }
    else {
        println!("Usage: {} <num_players> <max_value>",
                 prog_name.unwrap());
    }
}
