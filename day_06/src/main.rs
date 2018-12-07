use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;

fn dist(a: (i32,i32), b: (i32,i32)) -> i32 {
    let ((ax,ay),(bx,by)) = (a,b);
    let result = (ax-bx).abs() + (ay-by).abs();
    result
}

struct BoundingBox {
    origin: (i32,i32),
    end: (i32,i32),
}

struct BoundingBoxIter<'a> {
    boundingBox: &'a BoundingBox,
    cur: Option<(i32,i32)>
}


impl BoundingBox {
    fn new(origin: (i32,i32), end: (i32,i32)) -> BoundingBox {
        BoundingBox { origin: origin, end: end }
    }
}

impl<'a> IntoIterator for &'a BoundingBox {
    type Item = (i32,i32);
    type IntoIter = BoundingBoxIter<'a>;
    
    fn into_iter(self) -> Self::IntoIter {
        BoundingBoxIter { boundingBox: self, cur: Some(self.origin) }
    }
}
        

impl<'a> Iterator for BoundingBoxIter<'a> {
    type Item = (i32,i32);

    fn next(&mut self) -> Option<(i32,i32)> {
        let (x_min, _) = self.boundingBox.origin;
        let (x_max, y_max) = self.boundingBox.end;
        
        if let Some((mut x, mut y)) = self.cur {
            x += 1;
            if x > x_max {
                x = x_min;
                y += 1;
            }
            if y > y_max {
                self.cur = None;
            }
            else {
                self.cur = Some((x,y));
            }
        }
        self.cur
    }
}

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    let prog_name = args.next();

    if let Some(file_name) = args.next() {
        let mut file = File::open(&file_name)?;
        let mut reader = BufReader::new(file);

        let mut all_coords = Vec::<(i32,(i32,i32))>::new();
        let mut groups_by_label = HashMap::<i32,Vec<(i32,i32)>>::new();        
        let mut in_labels = HashSet::<i32>::new();
        let mut labelled_points = HashMap::<(i32,i32), i32>::new();
        
        let coords : Vec<(i32,i32)> = reader.lines().map(|line| {
            let coords : Vec<i32> = line.unwrap().split(", ").map(|x| { x.parse::<i32>().unwrap() }).collect();
            (coords[0], coords[1])
        }).collect();

        let coords_sorted_x : Vec<(i32,i32)> = { let mut c = coords.clone(); c.sort_by(|(x0,_),(x1,_)| { x0.cmp(x1) }); c };
        let coords_sorted_y : Vec<(i32,i32)> = { let mut c = coords.clone(); c.sort_by(|(_,y0),(_,y1)| { y0.cmp(y1) }); c };

        let (min_x, _) = coords_sorted_x.first().unwrap();
        let (max_x, _) = coords_sorted_x.last().unwrap();

        let (_, min_y) = coords_sorted_y.first().unwrap();
        let (_, max_y) = coords_sorted_y.last().unwrap();

        let boundingBox = BoundingBox::new( (*min_x, *min_y), (*max_x, *max_y) );

        //println!("Bounding Box = [({},{}),({},{})]", min_x, min_y, max_x, max_y);
        
        for (&(x, y), label) in coords.iter().zip(1..) {
            //println!("{}: {}, {}", label, x, y);
            all_coords.push((label, (x,y)));
            in_labels.insert(label);
        }

        for cell in (&boundingBox).into_iter() {
            let mut coords = all_coords.clone();
            coords.sort_by(|(_, a),(_, b)| {
                dist(cell, *a).cmp(&dist(cell, *b))
            });
            let lowest : Vec<(i32,(i32,i32))> = coords.into_iter().take(2).collect();

            let (a, a_cell) = lowest[0];
            let (_, b_cell) = lowest[1];
            
            if dist(cell, a_cell) != dist(cell, b_cell) {
                let label = a;
                // label owns this cell (it has the lowest distance)                
                let (x,y) = cell;
                //println!("({},{}) -> {}", x, y, label);
                groups_by_label.entry(label).and_modify(|group| {
                    group.push(cell);
                }).or_insert(vec![cell]);

                labelled_points.insert(cell, label);
                // disqualify labels with groups touching the edge
                if x == *min_x || x == *max_x || y == *min_y || y == *max_y {                
                    in_labels.remove(&label);
                }
            }
            else {                
                groups_by_label.entry(0).and_modify(|group| {
                    group.push(cell);
                }).or_insert(vec![cell]);
                labelled_points.insert(cell, 0);
            }
        }

        let max_label = in_labels.iter().fold(None, |max, label| {
            if let Some(max_label) = max {
                if groups_by_label[max_label].len() < groups_by_label[label].len() {
                    Some(label)
                }
                else {
                    Some(max_label)
                }
            }
            else {
                Some(label)
            }
        }).unwrap();

        // Want to print the graph out!      
        println!("Part 1: {} ({})", max_label, groups_by_label[max_label].len());

        //let max_dist = 32;
        let max_dist = 10000;
        
        let biggerBoundingBox = BoundingBox::new( (*max_x-max_dist, *max_y-max_dist),
                                                    (*min_x+max_dist, *min_y+max_dist) );
        let safe_cells = HashSet::<(i32,i32)>::new();
        
        let safe_cells : Vec<(i32,i32)> = biggerBoundingBox.into_iter().filter_map( |cell| {
            let sum_dist = all_coords.iter().fold(0, |sum, (_, coord)| { sum + dist(*coord, cell)});
            if sum_dist < max_dist {
                Some(cell)
            }
            else {
                None
            }
        }).collect();
        println!("Part 2: {}", safe_cells.len());
    }
    else {
        println!("Usage: {} <input_file>", prog_name.unwrap());
    }
    Ok(())        
}

