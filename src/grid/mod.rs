pub mod algorithm;

use colored::Colorize;

use crate::cell::Cell;

type Matrix<T> = [[T; 9]; 9];

pub struct Grid {
    matrix: Matrix<Cell>,
    current: Matrix<u8>,

    history: Vec<(usize, usize, Vec<u8>)>,

    pub completed: bool,
}

impl Grid {
    pub fn new() -> Self {
        let matrix_nums = [
            // [0, 2, 0, 5, 0, 1, 0, 9, 0],
            // [8, 0, 0, 2, 0, 3, 0, 0, 6],
            // [0, 3, 0, 0, 6, 0, 0, 7, 0],
            // [0, 0, 1, 0, 0, 0, 6, 0, 0],
            // [5, 4, 0, 0, 0, 0, 0, 1, 9],
            // [0, 0, 2, 0, 0, 0, 7, 0, 0],
            // [0, 9, 0, 0, 3, 0, 0, 8, 0],
            // [2, 0, 0, 8, 0, 4, 0, 0, 7],
            // [0, 1, 0, 9, 0, 7, 0, 6, 0]

            // [1, 2, 0, 5, 0, 0, 0, 0, 9],
            // [0, 0, 6, 0, 7, 0, 0, 4, 0],
            // [0, 0, 0, 2, 0, 0, 0, 5, 0],
            // [0, 0, 0, 6, 0, 0, 0, 9, 0],
            // [0, 0, 0, 0, 0, 0, 3, 0, 4],
            // [0, 9, 0, 7, 4, 0, 0, 0, 2],
            // [6, 0, 0, 0, 0, 0, 9, 0, 0],
            // [9, 7, 0, 0, 8, 0, 0, 0, 1],
            // [8, 0, 0, 0, 0, 1, 0, 0, 7],

            // [1, 2, 0, 3, 0, 0, 0, 0, 0],
            // [3, 4, 0, 0, 0, 0, 1, 0, 0],
            // [0, 0, 5, 0, 0, 0, 0, 0, 0],
            // [6, 0, 2, 4, 0, 0, 5, 0, 0],
            // [0, 0, 0, 0, 6, 0, 0, 7, 0],
            // [0, 0, 0, 0, 0, 8, 0, 0, 6],
            // [0, 0, 4, 2, 0, 0, 3, 0, 0],
            // [0, 0, 0, 0, 7, 0, 0, 0, 9],
            // [0, 0, 0, 0, 0, 9, 0, 8, 0]

            [2, 0, 9, 0, 0, 0, 0, 0, 0],
            [0, 8, 0, 0, 0, 0, 7, 0, 0],
            [0, 7, 0, 0, 0, 1, 5, 0, 0],
            [4, 1, 0, 0, 0, 0, 0, 3, 0],
            [0, 9, 0, 0, 0, 2, 0, 0, 0],
            [0, 0, 0, 7, 0, 0, 0, 0, 8],
            [0, 0, 0, 6, 0, 0, 3, 0, 0],
            [0, 0, 0, 8, 7, 0, 0, 1, 0],
            [0, 0, 0, 0, 3, 9, 0, 4, 0]
        ];

        let mut matrix = [[Cell::NONE; 9]; 9];

        for x in 0..9 {
            for y in 0..9 {
                matrix[x][y] = match matrix_nums[y][x] {
                    0 => Cell::NONE,
                    n => Cell::DEFAULT(n)
                };
            }
        }

        Self {
            matrix,
            current: [[0; 9]; 9],
            history: Vec::new(),
            completed: false,
        }
    }

    pub fn draw(&self) {
        for x in 0..9usize {
            if x == 0 || x % 3 == 0 {
                self.draw_line();
            }

            for y in 0..9usize {
                if y == 0 || y % 3 == 0 {
                    print!("|");
                } else {
                    print!(" ");
                }

                let val = self.matrix[y][x];

                self.draw_cell(val);

                if y == 8 {
                    print!("|");
                }
            }

            println!();

            if x == 8 {
                self.draw_line()
            }
        }
    }

    fn draw_line(&self) {
        for _ in 0..(9 * 2u8 + 1) {
            print!("-");
        }

        println!()
    }

    fn draw_cell(&self, cell: Cell) {
        fn get_formatted(n: u8) -> String {
            format!("{}", n)
        }

        match cell {
            Cell::NONE => print!(" "),
            Cell::DEFAULT(n) => print!("{}", get_formatted(n)),
            Cell::PROBABLE(n) => print!("{}", get_formatted(n).green()),
            Cell::ERROR(n) => print!("{}", get_formatted(n).red()),
        }
    }

    fn validate_n(&mut self, n: u8, x: usize, y: usize) -> bool {
        fn validate_cell(c: Cell, n: u8) -> bool {
            if let Cell::DEFAULT(v) = c {
                if v == n {
                    return false;
                }
            }

            if let Cell::PROBABLE(v) = c {
                if v == n {
                    return false;
                }
            }

            true
        }

        for c in 0..9 {
            if !validate_cell(self.matrix[c][y], n) {
                return false;
            }
        }

        for r in 0..9 {
            if !validate_cell(self.matrix[x][r], n) {
                return false;
            }
        }

        let xq = (x as f32 / 3.0).floor();
        let yq = (y as f32 / 3.0).floor();

        for x in 0..3 {
            for y in 0..3 {
                if !validate_cell(self.matrix[xq as usize * 3 + x][yq as usize * 3 + y], n) {
                    return false;
                }
            }
        }

        true
    }
}

