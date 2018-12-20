extern crate ncurses;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Debug)]
enum Track {
    Vertical,
    Horizontal,
    Crossing,
    LeftTurn,
    RightTurn,
}

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Clone)]
#[derive(Debug)]
enum CartDirection {
    Right,
    Left,
    Up,
    Down,
    Collision        
}

impl Track {
    fn try_from(c: char) -> Option<Self> {
        match c {
            '|' | '-' | '+' | '\\' | '/' => Some(Track::from(c)),
            _ => None
        }
    }

    fn to_char(&self) -> char {
        use Track::*;
        match self {
            Vertical => '|',
            Horizontal => '-',
            Crossing => '+',
            LeftTurn => '\\',
            RightTurn => '/'
        }                
    }

    fn to_string(&self) -> String {
        self.to_char().to_string()
    }

    fn do_choice(&mut self, dir: &mut CartDirection) {
        use Track::*;
        *self = match &self {
            LeftTurn => { dir.rotate_left(); Vertical },
            RightTurn => { dir.rotate_right(); LeftTurn },
            Vertical => RightTurn,
            _ => { panic!("Bad track type for 'choice' {:?}", self) }
        }
    }    
}

impl std::convert::From<char> for Track {
    fn from(c: char) -> Self {
        use Track::*;
        match c {
            '|' => Vertical,
            '-' => Horizontal,
            '+' => Crossing,
            '\\' => LeftTurn,
            '/' => RightTurn,
            _ => { panic!("invalid char for Track: {}", c); }
       }
    }
}

impl fmt::Display for Track {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_char().fmt(fmt)
    }
}

impl<'a> std::convert::From<&'a CartDirection> for Track {
    fn from(cart: &'a CartDirection) -> Self {
        use Track::*;
        use CartDirection::*;
        match cart {
            Down | Up => Vertical,
            Left | Right => Horizontal,
            Collision => panic!("Not valid source for Track")
       }
    }
}

#[derive(Eq)]
struct Cart((u32,u32), CartDirection, Track);

impl Cart {
    fn do_move(&mut self) {
        use CartDirection::*;
        
        let Cart((ref mut x, ref mut y), dir, _) = self;
        match dir {
            Down  => {*y = *y + 1;},
            Up    => {*y = *y - 1;},
            Right => {*x = *x + 1;},
            Left  => {*x = *x - 1;},
            Collision => {}
        };
    }

    fn do_rotate(&mut self, track: &Track) {
        use Track::*;
        use CartDirection::*;
        let Cart(_, ref mut dir, ref mut choice) = self;
        match track {
            LeftTurn => {
                match dir {
                    Up | Down => { dir.rotate_left(); },
                    Left | Right => { dir.rotate_right(); },
                    Collision => {}
                }
            },
            RightTurn => {
                match dir {
                    Up | Down => { dir.rotate_right(); },
                    Left | Right => { dir.rotate_left(); },
                    Collision => {}
                }
            },
            Crossing => { choice.do_choice(dir); }
            _ => {}
        };
    }
}

impl std::cmp::PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::PartialEq for Cart {
    fn eq(&self, other: &Cart) -> bool {
        let Cart((x0,y0),_,_) = self;
        let Cart((x1,y1),_,_) = other;

        x0 == x1 && y0 == y1
    }
}

impl std::cmp::Ord for Cart {
    fn cmp(&self, other: &Cart) -> std::cmp::Ordering {
        let Cart((x0,y0),_,_) = self;
        let Cart((x1,y1),_,_) = other;
        if y0 == y1 {
            x0.cmp(x1)
        }
        else {
            y0.cmp(y1)
        }
    }
}

impl CartDirection {
    fn try_from(c: char) -> Option<Self> {
        match c {
            '>' | '<' | '^' | 'v' => Some(CartDirection::from(c)),
            _ => None
        }
    }

    fn rotate_left(&mut self) {
        use CartDirection::*;
        *self = match &self {
            Down => Right,
            Up => Left,
            Right => Up,
            Left => Down,
            Collision => Collision                
        };
    }
    fn rotate_right(&mut self) {
        use CartDirection::*;
        *self = match &self {
            Down => Left,
            Up => Right,
            Right => Down,
            Left => Up,
            Collision => Collision
        };
    }

    fn to_char(&self) -> char {
        use CartDirection::*;
        match self {
            Down => 'v',
            Up => '^',
            Right => '>',
            Left => '<',
            Collision => 'X'
                
        }
    }

    fn to_string(&self) -> String {
        self.to_char().to_string()
    }
}

impl std::convert::From<char> for CartDirection {
    fn from(c: char) -> Self {
        use CartDirection::*;
        match c {
            '>' => Right,
            '<' => Left,
            '^' => Up,
            'v' => Down,
            _ => { panic!("invalid char for Cart: {}", c); }                
        }
    }
}

// Returns the indices of the carts which are colliding
fn check_collisions(cart_idx: usize,
                    carts: &Vec<Cart>) -> Option<Vec<usize>> {

    let Cart((x0, y0),_,_) = carts[cart_idx];
    for (idx, Cart((x,y),dir,_)) in (0..).zip(carts.iter()) {
        if x0 == *x && y0 == *y
            && cart_idx != idx
            && *dir != CartDirection::Collision { 
                let mut result = vec![cart_idx, idx];
                result.sort();
                result.reverse();
                return Some(vec![cart_idx, idx]);
        }
    }
    None
}

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    let prog_name = args.next();
    
    if let Some(file_name) = args.next() {
        let mut f = File::open(file_name)?;
        let mut input = String::new();

        f.read_to_string(&mut input)?;
        let mut grid : HashMap<(u32,u32),Track> = HashMap::new();
        let mut carts : Vec<Cart> = Vec::new();
        let mut width = 0;
        let mut height = 0;
        {
            let mut x = 0;
            let mut y = 0;
            
            for c in input.chars() {
                if c == '\n' {
                    y += 1;
                    x = 0;
                }
                else if c == ' ' {
                    x += 1;
                }
                else {
                    if let Some(track) = Track::try_from(c) {
                        grid.insert((x,y), track);
                    }
                    else if let Some(cart_dir) = CartDirection::try_from(c) {
                        grid.insert((x,y), Track::from(&cart_dir));
                        carts.push( Cart( (x,y), cart_dir, Track::LeftTurn) );
                    }
                    else {
                        panic!("Bad input character: {}", c);
                    }
                    x += 1;
                }
                width = std::cmp::max(width, x);
                height = std::cmp::max(height, y);
            }
        }
        println!("{} Carts found", carts.len());
        /* Debug display with ncurses */
        ncurses::initscr();
        ncurses::raw();
        ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        ncurses::erase();
        ncurses::mvprintw(0, 0, "Drawing grid...");
        for j in 0..height {
            for i in 0..width {
                ncurses::mv(j as i32 + 1, i as i32);
                if let Some(track) = grid.get(&(i, j)) {
                    ncurses::printw(&track.to_string());
                }
                else {
                    ncurses::printw(" ");
                }
            }
        }
        ncurses::refresh();                
        let mut step = 0;
        
        let mut crashes : Vec<(u32,u32)> = Vec::new();
        'main_loop: loop {
            
            std::thread::sleep(std::time::Duration::from_millis(10));
            ncurses::mvprintw(0, 0, "Step ");
            ncurses::printw(&step.to_string());
            ncurses::clrtoeol();
            carts.sort();
            for idx in 0..carts.len() {
                let (oldx,oldy) = {let Cart((x,y), _,_) = carts[idx]; (x, y)};

                carts[idx].do_move();
                
                if let Some(collisions) = check_collisions(idx, &carts) {
                    for i in collisions {
                        let Cart((x,y),ref mut dir,_) = carts[i];
                        crashes.push((x,y));
                        *dir = CartDirection::Collision;
                    }
                }
                
                let (x,y, dir) = {let Cart((x,y), ref dir,_) = carts[idx]; (x, y, dir.clone())};

                ncurses::mvprintw(y as i32 + 1, x as i32, &dir.to_string());

                if let Some(track) = grid.get(&(x,y)) {
                    carts[idx].do_rotate(track);
                }

                if let Some(old_track) = grid.get(&(oldx, oldy)) {
                    ncurses::mvprintw(oldy as i32 + 1,  oldx as i32,
                                      &old_track.to_string());
                }
            }
            carts.retain(|Cart((_,_), dir, _)| { *dir != CartDirection::Collision });

            ncurses::mvprintw(ncurses::LINES() - 1, 0, "Carts Left: ");            
            ncurses::printw(&carts.len().to_string());
            ncurses::clrtoeol();
            ncurses::refresh();
            step += 1;
            
            if carts.len() <= 1 {
                break 'main_loop;
            }
        }

        ncurses::mvprintw(ncurses::LINES() - 1, 0,
                          "Finished, press any key to continue...");
        ncurses::getch();
        ncurses::endwin();

        let (x,y) = crashes[0];
        println!("Part 1: {}, {}", x, y);

        for (idx, (x,y)) in (0..).zip(crashes.iter()) {
            println!("{}: ({},{})", idx, x, y);
        }
        println!("CARTS");
        for (idx, Cart((x,y),_,_)) in (0..).zip(carts.iter()) {
            println!("{}: ({},{})", idx, x, y);
        }
        if carts.len() > 0 {
            let Cart((x,y),_,_) = carts[0];
            println!("Part 2: {}, {}", x, y);
        }
    }
    else {
        println!("Usage: {} <input_file>", prog_name.unwrap());
    }    

    Ok(())
}




