
static mut GRID : [ [Option<i8>; 300]; 300] = [[None; 300]; 300];

fn usage(prog_name: &str) {
    println!("Usage: {} <grid_serial> [<x> <y>]", prog_name);
}

fn parse_next<'a, 'b, V, T: std::iter::Iterator<Item=String>>(t: &'a mut T) -> Option<V>
where V: std::str::FromStr
{
    if let Some(next) = t.next() {
        next.parse::<V>().ok()
    }
    else {
        None
    }
}

struct Cell (u32,u32);

fn cell_level(cell: Cell, serial: i32) -> i8 {
    let Cell(x, y) = cell;
    let (grid_x, grid_y) = ((x - 1) as usize, (y - 1) as usize);
    if let Some(value) = unsafe {GRID[grid_y][grid_x]} {
        value
    }
    else {
        let value = calculate_cell(cell, serial);
        unsafe {GRID[grid_y][grid_x] = Some(value)};
        value
    }
}

fn calculate_cell(cell: Cell, serial: i32) -> i8 {
    let Cell(x, y) = cell;
    let rack_id = x as i32 + 10;
    let mut level = rack_id * y as i32;
    level += serial;
    level *= rack_id;
    
    (((level / 100) % 10) - 5) as i8
}

fn square_power(cell: Cell, size: u32, serial: i32) -> i32 {
    let mut power : i32 = 0;
    let Cell(x, y) = cell;
    for j in 0..size {
        for i in 0..size {
            power += cell_level(Cell(x + i, y + j), serial) as i32;
        }
    }
    power
}
    

fn main() {
    let mut args = std::env::args();
    let prog_name = args.next().unwrap();
    
    if let Some(serial) = parse_next::<i32,_>(&mut args) {
        if let (Some(x_coord), Some(y_coord)) = (parse_next::<u32,_>(&mut args),
                                                 parse_next::<u32,_>(&mut args)) {
            println!("{}", cell_level(Cell(x_coord, y_coord), serial));
        }
        else {
            // part 1
            // run a kernel over all 3x3 squares
            let mut max_power : Option<i32> = None;
            let mut max_cell : Option<Cell> = None;
            let mut max_size : Option<u32> = None;
            
            for size in 1..=300 {
                println!("Size = {}", size);
                for j in 1..=(300 - size) {
                    for i in 1..=(300 - size) {
                        let power = square_power(Cell(i,j), size, serial);
                        if let Some(max_pow) = max_power {
                            if power > max_pow {
                                max_power = Some(power);
                                max_cell = Some(Cell(i,j));
                                max_size = Some(size);
                            }
                        }
                        else {                            
                            max_power = Some(power);
                            max_cell = Some(Cell(i,j));
                            max_size = Some(size);
                        }
                    }
                }
            }
            if let (Some(Cell(x, y)), Some(max_size), Some(max_power))
                = (max_cell, max_size, max_power) {
                println!("{},{},{} == {}", x, y, max_size, max_power); 
            }
            else {
                println!("Didn't find a max power?!");
            }
        }
    }
    else {
        usage(&prog_name);        
    }
}
