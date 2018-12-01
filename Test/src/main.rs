use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    let mut line = String::new();
    loop {
        let n = io::stdin().read_line(&mut line)?;
        if n == 0 {
            break;
        }
        io::stdout().write(line.as_bytes())?;
    }
    Ok(())
}
