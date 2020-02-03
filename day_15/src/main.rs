extern crate ncurses;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use std::fmt;


#[derive(PartialEq)]
#[derive(Clone)]
enum UnitType {
    Goblin,
    Elf
}

impl<'a> std::convert::From<&'a UnitType> for char {
    fn from(unit: &'a UnitType) -> char {
        use UnitType::*;
        match unit {
            Goblin => 'G',
            Elf => 'E'
        }
    }
}

const STARTING_HP : u32 = 200;
const ATTACK_POW : u32 = 3;

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Clone)]
struct Point(u32,u32);

struct Unit {
    unit_type: UnitType,
    coords: Point,
    hit_points: u32
}

impl Unit {
    fn new(unit_type: UnitType, point: Point) -> Self {
        Unit {
            unit_type: unit_type,
            coords: point,
            hit_points: STARTING_HP
        }
    }
}

impl<'a> std::convert::From<&'a Unit> for char {
    fn from(unit: &Unit) -> Self {
        char::from(&unit.unit_type)
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}({})", char::from(self), self.hit_points)
    }
}

enum Cell {
    Wall,
    Empty,
    Occupied(Unit),
}

impl Cell {
    fn new(c: char, p: Point) -> Cell {
        use Cell::*;
        match c {
            '#' => Wall,
            '.' => Empty,
            'G' => Occupied(Unit::new(UnitType::Goblin, p)),
            'E' => Occupied(Unit::new(UnitType::Elf, p)),
            _ => panic!("Invalid cell type: {}", c)
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Cell::*;
        match self {
            Wall => '#'.fmt(fmt),
            Empty => '.'.fmt(fmt),
            Occupied(ref unit) => write!(fmt, "{}", char::from(unit))
        }
    }
}

struct Board {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>
}

impl Board {
    fn new(cells: Vec<Vec<Cell>>) -> Board {
        let height = cells.len();
        assert!(height > 0);

        let width = cells[0].len();
        for row in cells.iter().skip(1) {
            assert!(row.len() == width);
        }

        Board { width: width, height: height, cells: cells }
    }

    fn parse<'a> (input: &'a mut std::io::Read) -> Board {
        let mut cells : Vec<Vec<Cell>> = Vec::new();

        let mut buf : [u8; 1] = [0];
        let mut cur_row : Vec<Cell> = Vec::new();
        while input.read_exact(&mut buf).is_ok() {
            let c = buf[0] as char;
            if c == '\n' {
                cells.push(cur_row);
                cur_row = Vec::new();
            }
            else {
                let point = Point(cur_row.len() as u32, cells.len() as u32);
                cur_row.push(Cell::new(c, point));
            }
        }

        Board::new(cells)
    }

    fn at<'a> (&'a self, point: &Point) -> &'a Cell {
        let Point(x,y) = point;
        &self.cells[*y as usize][*x as usize]
    }

    fn at_mut<'a> (&'a mut self, point: &Point) -> &'a mut Cell {
        let Point(x,y) = point;
        &mut self.cells[*y as usize][*x as usize]
    }

    fn units<'a>(&'a self) -> impl Iterator<Item = &'a Unit> {
        let mut result = Vec::<&'a Unit>::new();

        self.cells.iter().flat_map(|row| row.iter().filter_map(|cell| {
            match cell {
                Cell::Occupied(ref unit) => Some(unit),
                _ => None
            }
        }))
    }

    fn units_of_type<'a>(&'a self, unit_type: UnitType) -> impl Iterator<Item = &'a Unit> {
	self.units().filter(move |unit| { unit.unit_type == unit_type })
    }

    fn units_for_row<'a>(&'a self, row: usize) -> impl Iterator<Item = &'a Unit> {
        self.cells[row].iter().filter_map(|cell| {
            if let Cell::Occupied(u) = cell {
                Some(u)
            }
            else {
                None
            }
        })
    }

    fn neighbors<'a>(&'a self, point: Point) -> impl Iterator<Item = &'a Cell> {
	
    }
}


// Game Rules

enum Action<'a> {
    None,
    MoveTo(Point),
    Attack(&'a Unit)
}

impl Unit {
    fn targets<'a>(&self, board: &'a Board) -> impl Iterator<Item = &'a Self> {
        let self_type = self.unit_type.clone();
        board.units().filter(move |u| {u.unit_type != self_type})
    }
    fn in_range<'a>(&self, board:&'a Board) -> impl Iterator<Item = &'a Self> {
	let visited : HashSet<Point> = HashSet::new();
	self.targets(board).flat_map(move |unit| { 
	    if !visited.contains(
	})
    }
    fn decide_move(&self, board: &Board) -> Action {
	let opposite_type = match (self.unit_type) {
	    UnitType::Elf => UnitType::Goblin,
	    UnitType::Goblin => UnitType::Elf
	};
	
	let targets = self.targets(board);
	let in_range_map = HashMap::new();
    }

}

trait Drawable {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn at<'a>(&self, point: &'a Point) -> String;
    fn row_info(&self, row: usize) -> String;
}

trait Visuals<T> {
    fn setup<'a>(&mut self, what: &'a T);
    fn done<'a,'b>(&mut self, what: &'a T);
    fn draw<'a>(&mut self, what: &'a T);
}

struct Display {
    sleep: u64,
    interactive: bool,
}

impl Display {
    pub fn new(sleep: u64) -> Self {
        Display {
            sleep: sleep,
            interactive: false
        }
    }

    pub fn interactive(mut self) -> Self {
        self.interactive = true;
        self
    }
}

impl Drawable for Board {
    fn width(&self) -> usize { self.width }
    fn height(&self) -> usize { self.height }
    fn at<'a>(&self, point: &'a Point) -> String {
        let Point(x,y) = point;
        format!("{}", self.cells[*y as usize][*x as usize])
    }
    fn row_info(&self, row: usize) -> String {
        self.units_for_row(row).map(|unit| {
            format!("{}", unit) }
        ).collect::<Vec<String>>().join(",")
    }
}

impl<T> Visuals<T> for Display where T: Drawable {
    fn setup<'a>(&mut self, what: &'a T) {
        ncurses::initscr();
        ncurses::noecho();
        ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        // This would be a good place to set up windows if desired
    }

    fn done<'a,'b>(&mut self, what: &'a T) {
        ncurses::mvprintw(what.height() as i32, 0,
                          "Finished, press [enter] to exit...");
        loop {
            let c = ncurses::getch();
            if c == 10 { break; }
        }
        ncurses::endwin();
    }

    fn draw<'a>(&mut self, what: &'a T) {
        ncurses::erase();

        // Draw the board
        for i in 0..what.height() {
            for j in 0..what.width() {
                ncurses::mvprintw(i as i32, j as i32,
                                  &what.at(&Point(j as u32, i as u32)));
            }
        }

        // Draw the horizontal line
        ncurses::mvvline(0, (what.width() + 1) as i32, '|' as u32, what.width() as i32);

        // Draw the unit health
        for row in 0..what.height() {
            ncurses::mvprintw(row as i32, (what.width() + 3) as i32,
                              &what.row_info(row));
        }

        ncurses::refresh();
        if self.interactive {
            let _ = ncurses::getch();
        }
        else {
            std::thread::sleep(std::time::Duration::from_millis(self.sleep));
        }
    }
}

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    let prog_name = args.next();

    if let Some(file_name) = args.next() {
        let mut f = File::open(file_name)?;
        let mut game_board = Board::parse(&mut f);
        let mut d = Display::new(200);
        d.setup(&game_board);
        d.draw(&game_board);
        loop {

            // Next board state
	    for unit in game_board.units() {
		
	    }

            d.draw(&game_board);
            // Need an end condition
            
        }
        d.done(&game_board);
    }
    else {
        println!("Usage: {} <input_file>", prog_name.unwrap());
    }

    Ok(())
}

