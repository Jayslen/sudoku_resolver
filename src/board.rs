pub type Cell = Option<u8>;
pub type Row = [Cell; 3];
pub type Grid = [[Row; 3]; 3];
pub type Board = [Grid; 3];

const INDEX_LIMIT: u8 = 2;

pub struct Position {
    pub x: u8,
    pub y: u8,
    pub row: u8,
    pub cell: u8,
}

impl Position {
    pub fn new() -> Position {
        Position {
            x: 0,
            y: 0,
            row: 0,
            cell: 0,
        }
    }

    pub fn update_position(&mut self) {
        let Position { x, y, row, cell } = self;

        println!("x: {}, y: {}, row: {}, cell: {}", x, y, row, cell);

        //update cell and row
        if *cell < INDEX_LIMIT {
            *cell = Self::update_value(cell);
        } else if *row < INDEX_LIMIT {
            *cell = Self::update_value(cell);
            *row = Self::update_value(row);
        } else if *row == INDEX_LIMIT && *x < INDEX_LIMIT {
            *x = Self::update_value(x);
            *cell = Self::update_value(cell);
            *row = Self::update_value(row);
        } else {
            *x = Self::update_value(x);
            *y = Self::update_value(y);
            *cell = Self::update_value(cell);
            *row = Self::update_value(row);
        }
    }

    fn update_value(value: &u8) -> u8 {
        match value {
            2 => 0,
            _ => value + 1,
        }
    }
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
