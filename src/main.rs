use sudoku_solver::grid::Grid;
use sudoku_solver::algorithm::Algorithm;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut grid = Grid::new();
    let mut i = 0u128;

    while !grid.completed {
        i += 1;

        print!("\x1B[2J\x1B[1;1H");
        println!("{:18}|{:?}", i, start.elapsed());

        grid.update();
        grid.draw();

    }
}
