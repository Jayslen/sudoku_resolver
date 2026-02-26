use std::collections::HashSet;

use self::board::{GAME_BOARD, Position};
use self::values::{current_grid, x_axis, y_axis};

mod board;
mod values;

fn main() {
    let position = Position {
        y: 0,
        x: 0,
        row: 0,
        cell: 0,
    };
    let current_grid = current_grid(&GAME_BOARD, &position);
    let x_values = x_axis(&GAME_BOARD, &position);
    let y_values = y_axis(&GAME_BOARD, &position);
    println!("Current row values: {:?}", current_grid);
    println!("X axis values: {:?}", x_values);
    println!("Y axis values: {:?}", y_values);

    let unique_values: HashSet<u8> = HashSet::from_iter(
        current_grid
            .into_iter()
            .chain(x_values.into_iter())
            .chain(y_values.into_iter()),
    );
    println!("Unique values: {:?}", unique_values);
}

// #[cfg(test)]
// mod tests {
//     use crate::board::{GAME_BOARD as TEST_BOARD, Position};
//     // const TEST_BOARD: ;

//     #[test]
//     // get all values in the same x axis (column)
//     fn get_x_values_1() {
//         let row_values = get_x_value(
//             &TEST_BOARD,
//             Position {
//                 y: 0,
//                 x: 0,
//                 row: 0,
//                 cell: 0,
//             },
//         );
//         assert_eq!(row_values, HashSet::from([9, 5, 8, 7]));
//     }

//     #[test]
//     fn get_x_values_2() {
//         let row_values = get_x_value(
//             &TEST_BOARD,
//             Position {
//                 y: 0,
//                 x: 0,
//                 row: 0,
//                 cell: 1,
//             },
//         );
//         assert_eq!(row_values, HashSet::from([8, 3, 2, 9, 5]));
//     }

//     #[test]
//     fn get_x_values_3() {
//         let row_values = get_x_value(
//             &TEST_BOARD,
//             Position {
//                 y: 0,
//                 x: 0,
//                 row: 0,
//                 cell: 2,
//             },
//         );
//         assert_eq!(row_values, HashSet::from([5, 4, 8]));
//     }

//     #[test]
//     fn get_y_values_1() {
//         let row_values = get_y_values(
//             &TEST_BOARD,
//             Position {
//                 y: 0,
//                 x: 0,
//                 row: 0,
//                 cell: 0,
//             },
//         );
//         assert_eq!(row_values, HashSet::from([9, 1, 5, 7]));
//     }
//     #[test]
//     fn get_y_values_2() {
//         let row_values = get_y_values(
//             &TEST_BOARD,
//             Position {
//                 y: 0,
//                 x: 0,
//                 row: 0,
//                 cell: 1,
//             },
//         );
//         assert_eq!(row_values, HashSet::from([8, 5, 7, 2]));
//     }

//     #[test]
//     fn get_y_values_3() {
//         let row_values = get_y_values(
//             &TEST_BOARD,
//             Position {
//                 y: 0,
//                 x: 0,
//                 row: 0,
//                 cell: 2,
//             },
//         );
//         assert_eq!(row_values, HashSet::from([4, 6, 1]));
//     }

//     #[test]
//     fn get_y_values_4() {
//         let row_values = get_y_values(
//             &TEST_BOARD,
//             Position {
//                 y: 0,
//                 x: 2,
//                 row: 0,
//                 cell: 0,
//             },
//         );
//         assert_eq!(row_values, HashSet::from([9]));
//     }

//     #[test]
//     fn get_y_values_5() {
//         let row_values = get_y_values(
//             &TEST_BOARD,
//             Position {
//                 y: 0,
//                 x: 2,
//                 row: 0,
//                 cell: 2,
//             },
//         );
//         assert_eq!(row_values, HashSet::from([7, 5, 2, 8, 1, 6]));
//     }
// }
