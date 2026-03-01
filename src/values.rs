use crate::board::{Board, Position};

pub fn x_axis(board: &Board, position: &Position) -> Vec<u8> {
    let current_columns = board[position.grid_row as usize]; // taking grid_row[0]
    let mut values: Vec<u8> = Vec::new();

    for column in current_columns {
        let row_values = column[position.inner_row as usize].iter();
        row_values.for_each(|cell| {
            if let Some(value) = cell {
                values.push(*value);
            }
        });
    }
    values
}

// what matters here is the x and cell
pub fn y_axis(board: &Board, position: &Position) -> Vec<u8> {
    let mut values: Vec<u8> = Vec::new();
    for grid_row in board {
        let grid_col = grid_row[position.grid_col as usize].iter();
        grid_col.for_each(|cell| {
            if let Some(value) = cell[position.inner_col as usize] {
                values.push(value);
            }
        });
    }
    values
}

pub fn current_grid(board: &Board, position: &Position) -> Vec<u8> {
    let mut values: Vec<u8> = Vec::new();
    let current_grid = board[position.grid_row as usize][position.grid_col as usize];

    for row in current_grid {
        row.iter().for_each(|cell| {
            if let Some(value) = cell {
                values.push(*value);
            }
        });
    }
    values
}
