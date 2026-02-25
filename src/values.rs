use crate::{Board, Position};
use std::collections::HashSet;

pub fn x_axis(board: &Board, position: Position) -> HashSet<u8> {
    let current_y_grid = board[position.y];
    let mut row_values: HashSet<u8> = HashSet::new();

    for row in current_y_grid {
        for value in row[position.cell] {
            if let Some(val) = value {
                row_values.insert(val);
            }
        }
    }
    row_values
}

// what matters here is the x and cell
pub fn y_axis(board: &Board, position: Position) -> HashSet<u8> {
    let mut values: HashSet<u8> = HashSet::new();
    for col in board {
        for row in col[position.x] {
            if let Some(value) = row[position.cell] {
                values.insert(value);
            }
        }
    }

    values
}
