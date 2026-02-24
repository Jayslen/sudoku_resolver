use std::collections::HashSet;

type Cell = Option<u8>;
type Row = [Cell; 3];
type Grid = [[Row; 3]; 3];

type Board = [Grid; 3];

struct Position {
    x: usize,
    y: usize,
    row: usize,
    cell: usize,
}

fn main() {
    let game_board: Board = [
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
    let position = Position {
        y: 0,
        x: 1,
        row: 0,
        cell: 0,
    };
}

// to get this values i just use position.y and position.cell.
// I don't need to use the x or row position because we just need the x axis values
// so it does not matter in which row the users is in
fn get_x_value(board: &Board, position: Position) -> HashSet<u8> {
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
fn get_y_values(board: &Board, position: Position) -> HashSet<u8> {
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

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_BOARD: Board = [
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

    #[test]
    // get all values in the same x axis (column)
    fn get_x_values_1() {
        let row_values = get_x_value(
            &TEST_BOARD,
            Position {
                y: 0,
                x: 0,
                row: 0,
                cell: 0,
            },
        );
        assert_eq!(row_values, HashSet::from([9, 5, 8, 7]));
    }

    #[test]
    fn get_x_values_2() {
        let row_values = get_x_value(
            &TEST_BOARD,
            Position {
                y: 0,
                x: 0,
                row: 0,
                cell: 1,
            },
        );
        assert_eq!(row_values, HashSet::from([8, 3, 2, 9, 5]));
    }

    #[test]
    fn get_x_values_3() {
        let row_values = get_x_value(
            &TEST_BOARD,
            Position {
                y: 0,
                x: 0,
                row: 0,
                cell: 2,
            },
        );
        assert_eq!(row_values, HashSet::from([5, 4, 8]));
    }

    #[test]
    fn get_y_values_1() {
        let row_values = get_y_values(
            &TEST_BOARD,
            Position {
                y: 0,
                x: 0,
                row: 0,
                cell: 0,
            },
        );
        assert_eq!(row_values, HashSet::from([9, 1, 5, 7]));
    }
    #[test]
    fn get_y_values_2() {
        let row_values = get_y_values(
            &TEST_BOARD,
            Position {
                y: 0,
                x: 0,
                row: 0,
                cell: 1,
            },
        );
        assert_eq!(row_values, HashSet::from([8, 5, 7, 2]));
    }

    #[test]
    fn get_y_values_3() {
        let row_values = get_y_values(
            &TEST_BOARD,
            Position {
                y: 0,
                x: 0,
                row: 0,
                cell: 2,
            },
        );
        assert_eq!(row_values, HashSet::from([4, 6, 1]));
    }

    #[test]
    fn get_y_values_4() {
        let row_values = get_y_values(
            &TEST_BOARD,
            Position {
                y: 0,
                x: 2,
                row: 0,
                cell: 0,
            },
        );
        assert_eq!(row_values, HashSet::from([9]));
    }

    #[test]
    fn get_y_values_5() {
        let row_values = get_y_values(
            &TEST_BOARD,
            Position {
                y: 0,
                x: 2,
                row: 0,
                cell: 2,
            },
        );
        assert_eq!(row_values, HashSet::from([7, 5, 2, 8, 1, 6]));
    }
}
