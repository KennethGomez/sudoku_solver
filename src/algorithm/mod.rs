pub trait Algorithm {
    fn update(&mut self);

    fn next(&mut self) -> (usize, usize, Vec<u8>);

    fn back(&mut self) -> (usize, usize, Vec<u8>);
}
