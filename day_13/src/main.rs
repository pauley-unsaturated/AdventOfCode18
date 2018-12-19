extern crate ncurses;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
enum Track {
    Vertical,
    Horizontal,
    Crossing,
    LeftTurn,
    RightTurn,
}

#[derive(Clone)]
#[derive(Debug)]
enum CartDirection {
    Right,
    Left,
    Up,
    Down
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
            Down | Up  => Vertical,
            Left | Right => Horizontal
       }
    }
}

struct Cart((u32,u32), CartDirection, Track);

impl Cart {
    fn do_move(&mut self) {
        use CartDirection::*;
        
        let Cart((ref mut x, ref mut y), dir, _) = self;
        match dir {
            Down  => {*y = *y + 1;},
            Up    => {*y = *y - 1;},
            Right => {*x = *x + 1;},
            Left  => {*x = *x - 1;}
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
                    Left | Right => { dir.rotate_right(); }
                }
            },
            RightTurn => {
                match dir {
                    Up | Down => { dir.rotate_right(); },
                    Left | Right => { dir.rotate_left(); },                    
                }
            },
            Crossing => { choice.do_choice(dir); }
            _ => {}
        };
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
            Left => Down
        };
    }
    fn rotate_right(&mut self) {
        use CartDirection::*;
        *self = match &self {
            Down => Left,
            Up => Right,
            Right => Down,
            Left => Up
        };
    }

    fn to_char(&self) -> char {
        use CartDirection::*;
        match self {
            Down => 'v',
            Up => '^',
            Right => '>',
            Left => '<'
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
        let x;
        let y;
        
        'main_loop: loop {
            std::thread::sleep(std::time::Duration::from_millis(20));
            ncurses::mvprintw(0, 0, "Step ");
            ncurses::printw(&step.to_string());
            ncurses::clrtoeol();
            for (idx, Cart((x0,y0),_,_)) in (0..).zip(carts.iter()) {
                for Cart((x1,y1),_,_) in carts.iter().skip(idx + 1) {                
                    if x0 == x1 && y0 == y1 {
                        ncurses::mvprintw(*y0 as i32 + 1, *x0 as i32, "X");
                        ncurses::mvprintw(height as i32 + 1, 0, "Collision!");
                        ncurses::mvprintw(height as i32 + 1, 0, "[Press any key to exit]");
                        ncurses::getch();
                        x = *x0;
                        y = *y0;
                        println!("Part 1: {}, {}", x0, y0);
                        break 'main_loop;
                    }
                }
            }
            for cart in carts.iter_mut() {
                let (oldx,oldy) = {let Cart((x,y), _,_) = cart; (*x, *y)};
                cart.do_move();
                let (x,y, dir) = {let Cart((x,y), dir,_) = cart; (*x, *y, dir.clone())};
                ncurses::mvprintw(y as i32 + 1, x as i32, &dir.to_string());
                if let Some(track) = grid.get(&(x,y)) {
                    cart.do_rotate(track);
                }
                if let Some(old_track) = grid.get(&(oldx, oldy)) {
                    ncurses::mvprintw(oldy as i32 + 1,  oldx as i32,
                                      &old_track.to_string());
                }
                                      
            }
            ncurses::refresh();
            step += 1;
        }
        ncurses::endwin();
        println!("Part 1: {}, {}", x, y);
    }
    else {
        println!("Usage: {} <input_file>", prog_name.unwrap());
    }    

    Ok(())
}




