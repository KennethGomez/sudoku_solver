#[derive(Clone, Copy)]
pub enum Cell {
    NONE,
    DEFAULT(u8),
    PROBABLE(u8),
    ERROR(u8),
}

impl Cell {
    pub fn is_default(&self) -> bool {
        matches!(*self, Cell::DEFAULT(_))
    }

    pub fn is_probable(&self) -> bool {
        matches!(*self, Cell::PROBABLE(_))
    }
}