use crate::board::{Board, Position};

pub fn x_axis(board: &Board, position: &Position) -> Vec<u8> {
    let current_y_grid = board[position.y as usize];
    let mut values: Vec<u8> = Vec::new();

    for row in current_y_grid {
        let row_values = row[position.x as usize].iter();
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
    for col in board {
        let row = col[position.x as usize].iter();
        row.for_each(|cell| {
            if let Some(value) = cell[position.cell as usize] {
                values.push(value);
            }
        })
    }
    values
}

pub fn current_grid(board: &Board, position: &Position) -> Vec<u8> {
    let mut values: Vec<u8> = Vec::new();
    let current_grid = board[position.y as usize][position.x as usize];

    for row in current_grid {
        row.iter().for_each(|cell| {
            if let Some(value) = cell {
                values.push(*value);
            }
        });
    }
    values
}
