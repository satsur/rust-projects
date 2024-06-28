use core::fmt::Formatter;
use std::fmt::{Error, Display};

const BOARD_SIZE: u8 = 9;

#[derive(Debug)]
struct BoardConfiguration {
    state: [[u8; BOARD_SIZE as usize]; BOARD_SIZE as usize],
}

#[derive(Debug)]
struct BoardState {
    solved: bool,
    state: Vec<Vec<Cell>>,
}

impl BoardState {
    fn is_solved(&self) -> bool {
        for row in &self.state {
            for cell in row {
                if !cell.fixed { return false; }
            }
        }
        return true;
    }
}

#[derive(Debug)]
struct SudokuBoard {
    board: BoardState,
}

impl SudokuBoard {
    fn new(config: &BoardConfiguration) -> Self{
        Self {
            board: Self::board_state_from_config(config),
        }
    }
    
    fn board_state_from_config(config: &BoardConfiguration) -> BoardState{
        let mut state: Vec<Vec<Cell>> = vec![];
        for row in 0..config.state.len(){
            state.push(Vec::new());
            for col in 0..config.state[1].len() {
                let fixed = config.state[row][col] != 0;
                let cell_value = if fixed { Some(config.state[row][col]) } else { None };
                state[row].push(Cell::new(fixed, cell_value));
            }
        }
        return BoardState {
            solved: false,
            state
        }
    }
}

impl Display for SudokuBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for row in 0..self.board.state.len() {                              // Iterate through each row of board
            for col in 0..self.board.state[0].len() {                       // Each column
                if col + 1 & 3 == 0 {                                       // Every third column, print a vertical barrier (to separate squares)
                    write!(f, "| ");
                }
                write!(f, "{}", self.board.state[row][col]);                // Display the Cell (see Display impl for Cell)
            }
            if row != self.board.state.len() - 1 && (row + 1) % 3 == 0 {    // Every three rows (except last), print a horizontal barrier
                write!(f, "\n{}", "--".to_string().repeat(11));
            }
            write!(f, "\n");                                                // After every row, print a newline
        }
        Ok(())
    }

}

#[derive(Debug)]
struct Cell {
    fixed: bool,
    value: Option<u8>,          // None is empty cell
    possible_values: Vec<u8>,
}

impl Cell {
    // Possible values are always either 1-9 or just the cell value initially
    fn new(fixed: bool, value: Option<u8>) -> Self {
        Self {
            fixed: if value.is_some() { fixed } else { false },
            value,
            possible_values: if fixed { vec![value.unwrap()] } else { (1..=9).collect() }
        }
    }
    
    fn could_be(&self, n: &u8) -> bool {
        self.possible_values.contains(n)
    }
    
    fn remove_from_possible_values(&mut self, n: &u8) {
        if self.possible_values.contains(n) {
            let position = self.possible_values.iter().position(|&v| &v == n);
            if position.is_some() {
                self.possible_values.remove(position.unwrap());
            }
        }
    }
    
    fn add_to_possible_values(&mut self, n: &u8) {
        if !self.possible_values.contains(n) && *n > 0 && *n < BOARD_SIZE {
            self.possible_values.push(*n);
        }
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        // If value is set, print the value. Otherwise, just print an underscore (_) placeholder
        write!(f, "{}", if self.value.is_some() { self.value.unwrap().to_string() + " "} else { "_ ".to_string() })
    }
}

fn main() {
    // Board config -> Board state (Vec<Vec<Cell>> -> Stored in SudokuBoard
    let board_config = BoardConfiguration {
        state: [
                [0, 0, 0, 2, 6, 0, 7, 0, 1],
                [6, 8, 0, 0, 7, 0, 0, 9, 0],
                [1, 9, 0, 0, 0, 4, 5, 0, 0],
                [8, 2, 0, 1, 0, 0, 0, 4, 0],
                [0, 0, 4, 6, 0, 2, 9, 0, 0],
                [0, 5, 0, 0, 0, 3, 0, 2, 8],
                [0, 0, 9, 3, 0, 0, 0, 7, 4],
                [0, 4, 0, 0, 5, 0, 0, 3, 6],
                [7, 0, 3, 0, 1, 8, 0, 0, 0]
        ]
    };

    let board: SudokuBoard = SudokuBoard::new(&board_config);
    println!("{}", board);
}
