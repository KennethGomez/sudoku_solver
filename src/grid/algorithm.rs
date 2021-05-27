use crate::algorithm::Algorithm;
use crate::cell::Cell;
use crate::grid::Grid;

impl Algorithm for Grid {
    fn update(&mut self) {
        let (mut x, mut y, mut possibilities) = self.next();

        if self.completed == true {
            return;
        }

        self.history.push((x, y, possibilities.clone()));

        if possibilities.len() == 0 {
            let (nx, ny, np) = self.back();

            x = nx;
            y = ny;
            possibilities = np;
        }

        let current = self.current[x][y] as usize;

        let n = possibilities[current];

        self.matrix[x][y] = Cell::PROBABLE(n);
    }

    fn next(&mut self) -> (usize, usize, Vec<u8>) {
        let mut tx: Option<usize> = None;
        let mut ty: Option<usize> = None;
        let mut possibilities = vec![0u8; 9];

        for x in 0..9 {
            for y in 0..9 {
                let c = self.matrix[x][y];

                if c.is_probable() || c.is_default() {
                    continue;
                }

                let mut p: Vec<u8> = Vec::new();

                for i in 1..10 {
                    if self.validate_n(i, x, y) {
                        p.push(i);
                    }
                }

                if p.len() < possibilities.len() {
                    possibilities = p;
                    tx = Some(x);
                    ty = Some(y);
                }
            }
        }

        if tx.is_none() || ty.is_none() {
            self.completed = true;

            return (0, 0, possibilities);
        }

        (tx.unwrap(), ty.unwrap(), possibilities)
    }

    fn back(&mut self) -> (usize, usize, Vec<u8>) {
        let tx: Option<usize> = None;
        let ty: Option<usize> = None;
        let possibilities = vec![0u8; 9];

        if let Some((x_, y_, pos_)) = self.history.last() {
            let x = *x_;
            let y = *y_;
            let pos = pos_.clone();

            self.current[x][y] += 1;

            if self.current[x][y] as usize >= pos.len() {
                self.current[x][y] = 0;
                self.matrix[x][y] = Cell::NONE;

                self.history.pop();

                return self.back();
            }

            return (x, y, pos);
        }

        if tx.is_none() || ty.is_none() {
            panic!("Error finding next")
        }

        (tx.unwrap(), ty.unwrap(), possibilities)
    }
}
