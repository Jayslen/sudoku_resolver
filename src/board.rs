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

pub static GAME_BOARD: Board = [
    [
        [
            [Some(9), None, None],
            [None, Some(8), None],
            [None, Some(5), Some(4)],
        ],
        [
            [Some(5), None, Some(8)],
            [Some(3), None, Some(2)],
            [None, None, None],
        ],
        [
            [None, None, Some(7)],
            [Some(9), None, Some(5)],
            [None, Some(8), None],
        ],
    ],
    [
        [
            [None, Some(7), None],
            [Some(1), None, None],
            [Some(5), None, None],
        ],
        [
            [Some(6), Some(8), None],
            [None, Some(4), Some(4)],
            [Some(2), Some(1), Some(9)],
        ],
        [
            [None, Some(3), Some(2)],
            [None, None, Some(8)],
            [None, Some(6), None],
        ],
    ],
    [
        [
            [None, None, None],
            [Some(7), Some(2), Some(6)],
            [None, None, Some(1)],
        ],
        [
            [Some(9), None, Some(6)],
            [None, None, Some(1)],
            [Some(4), Some(7), None],
        ],
        [
            [None, None, Some(1)],
            [None, Some(4), None],
            [None, Some(5), Some(6)],
        ],
    ],
];
