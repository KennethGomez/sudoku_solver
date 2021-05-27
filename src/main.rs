use std::io;
use std::io::{Read, Write};
use std::time::Instant;

use sudoku_solver::algorithm::Algorithm;
use sudoku_solver::grid::Grid;

fn main() {
    let start = Instant::now();
    let mut grid = Grid::new();
    let mut i = 0u128;

    let enabled = ansi_term::enable_ansi_support();

    if enabled.is_err() {
        panic!("Error enabling ansi support");
    }

    while !grid.completed {
        i += 1;

        print!("\x1B[2J\x1B[1;1H");
        println!("{:11} iters | {:?}", i, start.elapsed());

        grid.update();
        grid.draw();
    }

    pause();
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "Program finished! Press any key to continue...").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}
