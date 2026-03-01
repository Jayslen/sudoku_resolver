use self::board::{Board, GameBoard};

mod board;
mod values;

fn main() {
    let mut game_board = GameBoard::new();

    solve(&mut game_board);

    print_board(&game_board);
}

fn solve(board: &mut Board) -> bool {
    let empty_cell = GameBoard::find_empty_cell(board);

    if let Some(coords) = empty_cell {
        // println!("Found empty cell at ({}, {})", coords.x, coords.y);
        for value in 1u8..=9 {
            if GameBoard::is_valid(board, &coords, &value) {
                let position = GameBoard::coords_to_position(&coords);
                // println!("{:?}", position);
                board[position.grid_row as usize][position.grid_col as usize]
                    [position.inner_row as usize][position.inner_col as usize] = Some(value);

                // print_board(board);
                if solve(board) {
                    return true;
                }
                board[position.grid_row as usize][position.grid_col as usize]
                    [position.inner_row as usize][position.inner_col as usize] = None;
            }
        }
    } else {
        return true;
    }
    false
}

pub fn print_board(board: &Board) {
    for big_row in 0..3 {
        for inner_row in 0..3 {
            if big_row > 0 && inner_row == 0 {
                println!("------+-------+------");
            }

            for big_col in 0..3 {
                for inner_col in 0..3 {
                    let cell = board[big_row][big_col][inner_row][inner_col];

                    match cell {
                        Some(value) => print!("{} ", value),
                        None => print!(". "),
                    }
                }

                if big_col < 2 {
                    print!("| ");
                }
            }

            println!();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::board::{Coords, GameBoard};
    use crate::values;

    #[test]
    fn test_position_conversion() {
        let coords = Coords { x: 2, y: 0 };
        let position = GameBoard::coords_to_position(&coords);
        assert_eq!(position.grid_row, 0);
        assert_eq!(position.grid_col, 0);
        assert_eq!(position.inner_row, 0);
        assert_eq!(position.inner_col, 2);
    }
    #[test]
    fn test_position_conversion_2() {
        let coords = Coords { x: 8, y: 2 };
        let position = GameBoard::coords_to_position(&coords);
        assert_eq!(position.grid_row, 0);
        assert_eq!(position.grid_col, 2);
        assert_eq!(position.inner_row, 2);
        assert_eq!(position.inner_col, 2);
    }

    #[test]
    fn test_position_conversion_3() {
        let coords = Coords { x: 2, y: 7 };
        let position = GameBoard::coords_to_position(&coords);
        assert_eq!(position.grid_row, 2);
        assert_eq!(position.grid_col, 0);
        assert_eq!(position.inner_row, 1);
        assert_eq!(position.inner_col, 2);
    }

    #[test]
    fn test_position_conversion_4() {
        let coords = Coords { x: 5, y: 1 };
        let position = GameBoard::coords_to_position(&coords);
        assert_eq!(position.grid_row, 0);
        assert_eq!(position.grid_col, 1);
        assert_eq!(position.inner_row, 1);
        assert_eq!(position.inner_col, 2);
    }

    #[test]
    fn get_values_1() {
        let board = GameBoard::new();
        let coords = Coords { x: 2, y: 0 };
        let position = GameBoard::coords_to_position(&coords);
        let x_values = values::x_axis(&board, &position);
        let y_values = values::y_axis(&board, &position);
        let grid_values = values::current_grid(&board, &position);

        assert_eq!(x_values, vec![9, 2, 4, 8]);
        assert_eq!(y_values, vec![3, 1, 5, 2, 7]);
        assert_eq!(grid_values, vec![9, 2, 3, 8, 1]);
    }

    #[test]
    fn get_values_2() {
        let board = GameBoard::new();
        let coords = Coords { x: 5, y: 3 };
        let position = GameBoard::coords_to_position(&coords);
        let x_values = values::x_axis(&board, &position);
        let y_values = values::y_axis(&board, &position);
        let grid_values = values::current_grid(&board, &position);

        assert_eq!(x_values, vec![5, 9, 1, 2]);
        assert_eq!(y_values, vec![1, 6, 9, 8, 3]);
        assert_eq!(grid_values, vec![9, 8, 6]);
    }

    #[test]
    fn get_values_3() {
        let board = GameBoard::new();
        let coords = Coords { x: 3, y: 3 };
        let position = GameBoard::coords_to_position(&coords);
        let x_values = values::x_axis(&board, &position);
        let y_values = values::y_axis(&board, &position);
        let grid_values = values::current_grid(&board, &position);

        assert_eq!(x_values, vec![5, 9, 1, 2]);
        assert_eq!(y_values, vec![7, 9, 6]);
        assert_eq!(grid_values, vec![9, 8, 6]);
    }
    #[test]
    fn valid_position() {
        let board = GameBoard::new();
        let coords = Coords { x: 2, y: 0 };

        assert!(!GameBoard::is_valid(&board, &coords, &4));
        assert!(GameBoard::is_valid(&board, &coords, &6));
        assert!(!GameBoard::is_valid(&board, &coords, &8));
    }

    // #[test]
    // fn find_empty_cell() {
    //     let board = GameBoard::new();
    //     let empty_cell = GameBoard::find_empty_cell(&board).expect("error");

    //     assert_eq!(empty_cell.x, 2);
    //     assert_eq!(empty_cell.y, 0);
    // }
}
