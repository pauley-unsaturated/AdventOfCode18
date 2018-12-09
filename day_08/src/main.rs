use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::iter::Iterator;

// Children , Metadata
struct Node (Vec<Node>, Vec<u32>, u32);

struct NodeIter<'a>(Vec<Box<dyn Iterator<Item=&'a Node> + 'a>>);

impl<'a> NodeIter<'a> {
    fn new(node : &'a Node) -> NodeIter<'a> {
        NodeIter(vec![Box::new(vec![node].into_iter())])
    }
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<&'a Node> {
        // pre-order
        let NodeIter(ref mut stack) = self;
        loop {
            if let Some(mut top) = stack.pop() {
                // top is a boxed iterator
                if let Some(node) = top.next() {
                    let Node(children, _, _) = node;
                    stack.push(top);
                    stack.push(Box::new(children.iter()));
                    return Some(node);
                }            
            }
            else {
                return None;
            }
        } 
    }
}

fn read_node<I: Iterator<Item = u32>> (input: &mut I) -> Node
{
    let num_children = input.next().unwrap();
    let num_entries =  input.next().unwrap();
    let mut children = Vec::new();
    for _ in 0..num_children {
        children.push(read_node(input));
    }
    let mut entries = Vec::new();
    for _ in 0..num_entries {
        let entry = input.next().unwrap();
        entries.push(entry);
    }
    let value = if num_children == 0 {
        entries.iter().fold(0, |sum, x| {sum + x})
    }
    else {
        entries.iter().fold(0, |sum, x| {
            let idx = (*x as usize) - 1;
            sum + if idx < children.len() {
                let Node(_, _, value) = children[idx as usize];
                value
            }
            else {
                0
            }
        })
    };        
    Node(children, entries, value)
 }

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    let prog_name = args.next();

    if let Some(file_name) = args.next() {

    let mut f = File::open(file_name)?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;

        let mut nums = contents.split(" ").filter_map(|s| {
            s.parse::<u32>().ok()
        }).into_iter();

        // num_children, children, num_entries, entries
        let tree = read_node(&mut nums);
        let tree_iter = NodeIter::new(&tree);
        let mut sum = 0;
        for Node(_, entries, _) in tree_iter {
            for e in entries.iter() {
                sum += e;
            }
        }
        println!("Part 1: {}", sum);
        
        let Node(_, _, root_value) = tree;
        println!("Part 2: {}", root_value);
            
    }
    else {
        println!("Usage: {} <input_file>", prog_name.unwrap());
    }
    Ok(()) 
}
