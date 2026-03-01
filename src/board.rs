use crate::values::{current_grid, x_axis, y_axis};
use std::collections::HashSet;

pub type Cell = Option<u8>;
pub type Row = [Cell; 3];
pub type Grid = [[Row; 3]; 3];
pub type Board = [Grid; 3];

#[derive(Debug)]
pub struct Coords {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug)]
pub struct Position {
    pub grid_row: u8,
    pub grid_col: u8,
    pub inner_row: u8,
    pub inner_col: u8,
}

pub struct GameBoard {
    pub board: Board,
    pub position: Position,
}

impl GameBoard {
    pub fn coords_to_position(coords: &Coords) -> Position {
        let Coords { x, y } = coords;

        Position {
            grid_row: (y / 3) as u8,
            grid_col: (x / 3) as u8,
            inner_row: (y % 3) as u8,
            inner_col: (x % 3) as u8,
        }
    }

    pub fn is_valid(board: &Board, coords: &Coords, value: &u8) -> bool {
        let values: HashSet<u8> = HashSet::from_iter(
            x_axis(board, &GameBoard::coords_to_position(coords))
                .into_iter()
                .chain(y_axis(board, &GameBoard::coords_to_position(coords)))
                .chain(current_grid(board, &GameBoard::coords_to_position(coords))),
        );

        if values.contains(value) {
            return false;
        }
        true
    }

    pub fn find_empty_cell(board: &Board) -> Option<Coords> {
        for grid_col in 0..3 {
            for grid_row in 0..3 {
                for inner_row in 0..3 {
                    for inner_col in 0..3 {
                        if board[grid_row][grid_col][inner_row][inner_col].is_none() {
                            return Some(Coords {
                                x: (grid_col * 3 + inner_col) as u8,
                                y: (grid_row * 3 + inner_row) as u8,
                            });
                        }
                    }
                }
            }
        }
        None
    }

    pub fn new() -> Board {
        [
            [
                [
                    [Some(9), Some(2), None],
                    [None, None, Some(3)],
                    [Some(8), None, Some(1)],
                ],
                [
                    [None, Some(4), None],
                    [None, None, Some(1)],
                    [Some(7), None, Some(6)],
                ],
                [
                    [Some(8), None, None],
                    [None, Some(5), None],
                    [Some(3), Some(4), Some(9)],
                ],
            ],
            [
                [
                    [None, None, Some(5)],
                    [Some(2), Some(1), None],
                    [None, None, None],
                ],
                [
                    [None, None, Some(9)],
                    [None, None, Some(8)],
                    [None, Some(6), None],
                ],
                [
                    [Some(1), None, Some(2)],
                    [None, Some(6), Some(4)],
                    [Some(5), None, None],
                ],
            ],
            [
                [
                    [Some(5), Some(6), None],
                    [None, Some(4), Some(2)],
                    [Some(1), None, Some(7)],
                ],
                [
                    [Some(9), Some(1), Some(3)],
                    [Some(6), None, None],
                    [None, None, None],
                ],
                [
                    [Some(4), None, None],
                    [None, Some(1), None],
                    [Some(6), None, Some(3)],
                ],
            ],
        ]
    }
}
