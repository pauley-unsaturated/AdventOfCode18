extern crate ncurses;
extern crate priority_queue;

use std::io;
use std::fs::File;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::fmt;
use std::{thread, time};
use std::panic;


macro_rules! displayln {
    ($str:expr) => ({
       get_display().println($str)
    });
    ($fmt:expr, $($arg:tt)+) => ({
        get_display().println(&format!($fmt, $($arg)+))
    });
}


use priority_queue::PriorityQueue;

#[derive(PartialEq)]
#[derive(Clone)]
enum UnitType {
    Goblin,
    Elf
}

impl UnitType {
    fn opposite(&self) -> UnitType {
        match self {
            UnitType::Goblin => UnitType::Elf,
            UnitType::Elf => UnitType::Goblin
        }
    }
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
#[derive(PartialOrd)]
#[derive(Hash)]
#[derive(Clone)]
#[derive(Copy)]
struct Point(u32,u32);

impl Ord for Point {
    fn cmp(&self, other : &Point) -> Ordering {
        match self.1.cmp(&other.1) {
            Ordering::Equal => self.0.cmp(&other.0),
            ordering => ordering
        }
    }
}

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

    fn parse<'a> (input: &'a mut dyn std::io::Read) -> Board {
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

    fn at<'a> (&'a self, point: Point) -> &'a Cell {
        let Point(x,y) = point;
        &self.cells[y as usize][x as usize]
    }

    fn at_mut<'a> (&'a mut self, point: Point) -> &'a mut Cell {
        let Point(x,y) = point;
        &mut self.cells[y as usize][x as usize]
    }

    fn units<'a>(&'a self) -> impl Iterator<Item = &'a Unit> {
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

    fn is_over(&self) -> bool {
        let mut some_elves = false;
        let mut some_goblins = false;
        for row in self.cells.iter() {
            for cell in row.iter() {
                if let Cell::Occupied(ref unit) = cell {
                    match unit.unit_type {
                        UnitType::Elf => {some_elves = true;},
                        UnitType::Goblin => {some_goblins = true;}
                    }
                    if some_elves && some_goblins {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    fn neighbors(&self, point: &Point) -> impl Iterator<Item = Point> {
        let end_x = (self.width - 1) as u32;
        let end_y = (self.height - 1) as u32;
	match point {
            Point(0, 0) => vec![Point(1,0), Point(0,1)],
            Point(0, y) if (*y == end_y) =>
                vec![Point(0, y - 1), Point(1, *y)],
            Point(0, y) => vec![Point(0, y - 1), Point(1, *y), Point(0, y + 1)],
            Point(x, 0) if (*x == end_x) =>
                vec![Point(x - 1, 0), Point(*x, 1)],
            Point(x, 0) =>
                vec![Point(x - 1, 0), Point(x + 1, 0), Point(*x, 1)],
            Point(x, y) if (*x == end_x && *y == end_y ) =>
                vec![Point(*x, y - 1), Point(x - 1, *y)],
            Point(x, y) if (*x == end_x) =>
                vec![Point(*x, y - 1), Point(x - 1, *y), Point(*x, y + 1)],
            Point(x, y) if (*y == end_y) =>
                vec![Point(*x, y - 1), Point(x - 1, *y), Point(*x + 1, *y)],
            Point(x, y) => vec![Point(*x, y - 1), Point(x - 1, *y),
                                Point(x + 1, *y), Point(*x, *y + 1)],
        }.into_iter()
    }

    fn move_cell(&mut self, from: Point, to: Point) -> () {
        unsafe {
            let from : *mut Cell = self.at_mut(from);
            let to : *mut Cell = self.at_mut(to);
            std::mem::swap(&mut *from, &mut *to);
        }
    }


    fn perform_action(&mut self, action: Action) -> Option<Point> {
        match action {
            Action::MoveTo(from,to) => {
                if let Cell::Empty = self.at(to) { /* Unoccupied, OK */ }
                else {
                     panic!("Attempting to move to occupied ({},{})!", to.0, to.1);
                }
                if let Cell::Occupied(ref mut unit) = self.at_mut(from) {
                    unit.coords = to
                }
                else {
                     panic!("Attempting to move from non-occupied ({},{})!", from.0, from.1);
                }

                self.move_cell(from, to);
                return Some(to);
            },
            Action::Attack(from, to) => {
                if let Cell::Occupied(_) = self.at(from) { /* OK */ }
                else {
                    panic!("Attempting to use an unoccupied cell to attack ({},{})!", from.0, from.1);
                }
                let to = self.at_mut(to);
                let mut dead = false;
                if let Cell::Occupied(ref mut to) = to {
                    if to.hit_points > ATTACK_POW {
                        to.hit_points -= ATTACK_POW;
                    }
                    else {
                        // They ded
                        to.hit_points = 0;
                        dead = true;
                    }
                }
                else {
                    panic!("Attempting to attack an unoccupied cell!");
                }

                if dead {
                    let mut empty = Cell::Empty;
                    std::mem::swap(to, &mut empty);
                }
            }
            Action::None => {}
        }
        None
    }
}


// Game Rules

enum Action {
    None,
    MoveTo(Point,Point),
    Attack(Point,Point)
}

impl Unit {
    fn targets<'a>(&self, board: &'a Board) -> impl Iterator<Item = &'a Self> {
        board.units_of_type(self.opposite_type())
    }

    fn opposite_type(&self) -> UnitType {
        self.unit_type.opposite()
    }

    fn in_range<'a>(&self, board: &'a Board) -> impl Iterator<Item = Point> + 'a {
        self.targets(board).flat_map(move |unit| {
            displayln!("Neighbors of {},{}", unit.coords.0, unit.coords.1);
            board.neighbors(&unit.coords).filter_map(move |point| {
                //displayln!("{},{}", point.0, point.1);
                match board.at(point) {
                    Cell::Empty => Some(point),
                    _ => None
                }
            })
        })
    }

    fn decide_attack(&self, board: &Board) -> Option<Action> {
        let mut lowest_hp : Option<u32> = None;
        let mut action : Option<Action> = None;

        for point in board.neighbors(&self.coords) {
            match board.at(point) {
                Cell::Occupied(ref unit) => {
                    if unit.unit_type == self.opposite_type() {
                        if lowest_hp.is_some() {
                            if lowest_hp.expect("??") > unit.hit_points {
                                lowest_hp = Some(unit.hit_points);
                                action = Some(Action::Attack(self.coords, point));
                            }
                        }
                        else {
                            lowest_hp = Some(unit.hit_points);
                            action = Some(Action::Attack(self.coords, point));
                        }
                    }
                }
                _ => {}
            }
        }
        action
    }

    fn decide_move(&self, board: &Board) -> Action {
        // Check for attack
        if self.decide_attack(board).is_some() {
            return Action::None;
        }
        // Otherwise determine target cell and attempt to move towards it
        #[derive(Eq)]
        #[derive(PartialEq)]
        #[derive(Hash)]
        struct Weight(u32,Point);
        impl Ord for Weight {
            fn cmp(&self, other: &Weight) -> Ordering {
                match self.0.cmp(&other.0) {
                    Ordering::Equal => other.1.cmp(&self.1),
                    Ordering::Less => Ordering::Greater,
                    Ordering::Greater => Ordering::Less
                }
            }
        };

        impl PartialOrd for Weight {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut ancestor : HashMap<Point,Point> = HashMap::new();

        let move_targets : HashSet<Point> = self.in_range(board).collect();
        let possible_moves : HashSet<Point> = board.neighbors(&self.coords).collect();

        let mut active_targets : PriorityQueue<Point,Weight> = PriorityQueue::new();

        if move_targets.is_empty() {
            displayln!("No Targets!");
            return Action::None
        }
        for target in move_targets.iter() {
            displayln!("Target {},{}", target.0, target.1);
        }

        // Currently, this uses flood-fill.. it should use A*
        active_targets.push(self.coords.clone(), Weight(0, self.coords.clone()));
        while let Some((next, weight)) = active_targets.pop() {
            displayln!("Looking at {},{} ({})", (weight.1).0, (weight.1).1, weight.0);
            if move_targets.contains(&next) {
                // Reached one of the targets
                // need the first move on the way to the target
                displayln!("Headed for {},{}({})", (weight.1).0, (weight.1).1, weight.0);
                let mut the_move = next;
                while !possible_moves.contains(&the_move) {
                    match ancestor.get(&the_move) {
                        None => panic!("Ancestor not found!"),
                        Some(previous_move) => {the_move = previous_move.clone();}
                    }
                }
                return Action::MoveTo(self.coords.clone(), the_move);
            }
            else {
                // push the neighbors that we haven't visited,
                // keeping track of how we got there
                let empty_neighbors = board.neighbors(&next).into_iter().filter(|point| {
                    if let Cell::Empty = board.at(*point) {true}
                    else {false}
                });
                for neighbor in empty_neighbors {
                    if !ancestor.contains_key(&neighbor) {
                        //println!("Pushing {},{}({})", neighbor.0, neighbor.1, weight.0 + 1);
                        ancestor.insert(neighbor, next);
                        let result = active_targets.push(neighbor,
                                                         Weight(weight.0 + 1, neighbor));
                        if let Some(weight) = result {
                            panic!("Duplicate priorities: {},{}({})",
                                   (weight.1).0, (weight.1).1, weight.0);
                        }
                        get_display().overlay(neighbor, "+");
                    }
                }
            }
        }
        return Action::None;
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
    output_line: u32,
    output_start: u32,
    interactive: bool
}

impl Display {
    pub fn new(sleep: u64) -> Self {
        Display {
            sleep: sleep,
            output_line: 0,
            output_start: 0,
            interactive: false
        }
    }

    pub fn interactive(mut self) -> Self {
        self.interactive = true;
        self
    }

    pub fn overlay(&mut self, p: Point, s: &str) {
        ncurses::mvprintw(p.1 as i32, p.0 as i32, s);
        ncurses::refresh();
    }

    pub fn clear_output(&mut self) {
        self.output_line = 0;
        ncurses::mvprintw(self.output_start as i32, 0, " ");
        ncurses::clrtobot();
    }

    pub fn println(&mut self, s: &str) {
        ncurses::mvprintw((self.output_line + self.output_start) as i32, 0, s);
        self.output_line += 1;
    }

    pub fn pause(&self) {
        if self.interactive {
            let _ = ncurses::getch();
        }
        else {
            std::thread::sleep(std::time::Duration::from_millis(self.sleep));
        }
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

        self.output_start = what.height() as u32 + 1;
        // This would be a good place to set up windows if desired
        let old_hook = panic::take_hook();
        panic::set_hook(Box::new(move |arg| {
            // End the window so that the panic message doesn't get garbled
            ncurses::endwin();
            old_hook(arg);
        }));
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
        //self.pause();
    }
}


static mut WRAPPED_DISPLAY : Option<Display> = None;

fn set_display(d : Display) -> () {
    let mut replacement = Some(d);
    unsafe {
        std::mem::swap(&mut WRAPPED_DISPLAY, &mut replacement);
    }
}

fn get_display() -> &'static mut Display {
    unsafe {
        if let Some(ref mut d) = WRAPPED_DISPLAY {
            let p : *mut Display = d;
            &mut (*p)
        }
        else { panic!("no display was set!")}
    }
}

fn main() -> io::Result<()> {
    let mut args = std::env::args();
    let prog_name = args.next();

    if let Some(file_name) = args.next() {
        let mut f = File::open(file_name)?;
        let mut game_board = Board::parse(&mut f);
        //set_display(Display::new(10).interactive());
        set_display(Display::new(10));

        let d = get_display();
        d.setup(&game_board);
        d.draw(&game_board);

        let mut num_rounds = 0;
        loop {
            d.clear_output();
            // Next board state
            let unit_coords = game_board.units().map(|unit| {
                unit.coords
            }).collect::<Vec<Point>>();

	    for unit_coord in unit_coords {
                // Move phase
                displayln!("Decide move");
                let action = {
                    let cell = game_board.at(unit_coord);
                    if let Cell::Occupied(unit) = cell {
                        unit.decide_move(&game_board)
                    }
                    else {
                        displayln!("No Move");
                        Action::None
                    }
                };
                displayln!("Move Decided");

                // Perform move
                let new_pos = game_board.perform_action(action);
                d.draw(&game_board);

                // Attack phase
                let attack_pos = new_pos.unwrap_or(unit_coord);
                displayln!("Attack Pos = {}, {}", attack_pos.0, attack_pos.1);
                let cell = game_board.at(attack_pos);

                if let Cell::Occupied(unit) = cell {
                    if let Some(attack) = unit.decide_attack(&game_board) {
                        game_board.perform_action(attack);
                    }
                }
                else {
                    displayln!("No Attack");
                }
                d.clear_output();
	    }
            d.draw(&game_board);
            d.pause();
            if game_board.is_over() { break; }
            else { num_rounds += 1; }
        }
        let hit_points = game_board.units().fold(0, {|acc, unit| acc + unit.hit_points});
        let outcome = num_rounds * hit_points;
        displayln!("Outcome: {} + {} = {}", num_rounds, hit_points, outcome);
        ncurses::getch();

        d.done(&game_board);
    }
    else {
        println!("Usage: {} <input_file>", prog_name.unwrap());
    }

    Ok(())
}

