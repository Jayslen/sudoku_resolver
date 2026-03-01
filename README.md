# Sudoku resolver algorithm in rust

This is a tiny project that i build to pratice Rust since i am learning it. The algorithm is pretty simple, it uses backtracking to solve the sudoku puzzle. It is not the most efficient algorithm, but it is easy to understand and implement.

Doing the project i learned about backtracking, and honestly it is a pretty cool algorithm.

The structure of the board i implemented was:

```rust
pub type Cell = Option<u8>;
pub type Row = [Cell; 3];
pub type Grid = [[Row; 3]; 3];
pub type Board = [Grid; 3];
```

Is an 3x3 array that represents the rows, inside each row we have another 3x3 array that represents the columns, and inside each column we have another 3x3 array that represents the cells. Each cell can be either a number from 1 to 9 or None if it is empty.

Then in order to find a cell the function takes a x,y position that gets translate into the board structure.
