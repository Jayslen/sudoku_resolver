pub type Cell = Option<u8>;
pub type Row = [Cell; 3];
pub type Grid = [[Row; 3]; 3];

pub type Board = [Grid; 3];

pub struct Position {
    pub x: usize,
    pub y: usize,
    pub row: usize,
    pub cell: usize,
}
