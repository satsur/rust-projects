// Standard 9x9 Sudoku

#[derive(Debug)]
struct Cell {
    fixed: bool,
    value: Option<u8>,          // Option::None is empty cell
}

impl Cell {
    // Possible values are always either 1-9 or just the cell value initially
    fn new(fixed: bool, value: Option<u8>) -> Self {
        Self {
            fixed: if value.is_some() { fixed } else { false },
            value,
        }
    }
}

#[derive(Debug)]
struct BoardConfiguration {
    state: [[u8; Board::BOARD_SIZE as usize]; Board::BOARD_SIZE as usize],
}

impl BoardConfiguration {
    // Converts a board config (2d array) into 2d vec of cells which we can manipulate more easily
    fn board_state_from_config(config: &BoardConfiguration) -> Vec<Vec<Cell>> {
        let mut state: Vec<Vec<Cell>> = vec![];
        for row in 0..config.state.len() {
            state.push(vec![]);
            for col in 0..config.state[0].len() {
                let current: u8 = config.state[row][col];
                let fixed: bool = current != 0;
                state[row].push(Cell::new(fixed, if fixed {Some(current)} else {None}));
            }
        }
        state
    }
}

#[derive(Debug)]
struct Board {
    grid: Vec<Vec<Cell>>,
}

impl Board {

    const BOARD_SIZE: usize = 9;
    const BOX_SIZE: usize = 3;
    const LOWEST_VALUE: u8 = 1;
    const HIGHEST_VALUE: u8 = 9;
    
    fn new(config: BoardConfiguration) -> Self {
        Self {
            grid: BoardConfiguration::board_state_from_config(&config),
        }
    }
    
    fn is_solved(&self) -> bool {
        for row in &self.grid {
            for cell in row {
                if !cell.fixed { return false; }
            }
        }
        true
    }
    
    fn val_in_row(&self, val: &u8, row: &usize) -> bool {
        for cell in &self.grid[*row] {
            if cell.value != None && cell.value.unwrap() == *val {
                // println!("Row check failed! Cell was {}", cell.value.unwrap());
                return true;
            }
        }
        false
    }
    
    fn val_in_col(&self, val: &u8, col: &usize) -> bool {
        for row in &self.grid {
            let cell = row[*col].value;
            if cell != None && cell.unwrap() == *val {
                // println!("Col check failed! Cell was {}", cell.unwrap());
                return true;
            }
        }
        false
    }
    
    fn val_in_box(&self, val:&u8, row: &usize, col: &usize) -> bool {
        // First identify which box it's in
        
        let cell_in_box = (*row % 3, *col % 3); // (row_in_box, col_in_box)
        let box_top_left= (*row - cell_in_box.0, *col - cell_in_box.1); // (row, col) of top left corner of the box
        
        // Then check all the cells in that box
        for row_increment in 0..Board::BOX_SIZE as usize {
            for col_increment in 0..Board::BOX_SIZE as usize {
                let cell = self.grid[ box_top_left.0 + row_increment ][ box_top_left.1 + col_increment ].value;
                if cell != None && cell.unwrap() == *val {
                    // println!("Box check failed! Cell was {}", cell.unwrap());
                    return true;
                }
            }
        }
        false
    }
    
    fn is_valid(&self, val: &u8, row: &usize, col: &usize) -> bool {
        !self.val_in_row(val, row) &&
        !self.val_in_col(val, col) && 
        !self.val_in_box(val, row, col)
    }
    
    fn display(&self) {
        for row in 0..self.grid.len() {
            if row % 3 == 0 { println!("-------------------------"); }
            for col in 0..self.grid[0].len() {
                let cell = &self.grid[row][col];
                if col % 3 == 0 {
                    print!("| ");
                }
                if cell.value != None {
                   print!("{}", cell.value.unwrap());
                } else {
                   print!("_");
                }
                print!(" ")
           }
           println!("|");
       }
       println!("-------------------------"); 
    }
    
    // Function to get coordinates of the cell X behind your current cell
    fn find_previous_unfixed_cell(&self, current: (usize, usize)) -> Option<(usize, usize)> {
        let mut cell_num = current.0 * current.1 + current.1;
        loop {
            if cell_num == 0 {
                return None;
            }
            let new_cell_num = cell_num - 1;
            let cell = (new_cell_num / Board::BOARD_SIZE as usize, new_cell_num % Board::BOARD_SIZE as usize);
            if !self.grid[cell.0][cell.1].fixed { 
                return Some(cell); 
            } else {
                cell_num = new_cell_num;
            }
        }
    }
    
    fn solve(&mut self, starting_row: usize, starting_col: usize) -> bool {
        for row in 0..self.grid.len() {
            for col in 0..self.grid[0].len() {
                // println!("On row {} and col {}", row, col);
                if self.grid[row][col].fixed { continue; }  // Fixed cells are set in stone

                let mut possible_value = self.grid[row][col].value.unwrap_or(Self::LOWEST_VALUE.clone());
                loop {
                    if possible_value == Self::HIGHEST_VALUE + 1 {
                        possible_value = Self::LOWEST_VALUE.clone();
                        let new_cell = self.find_previous_unfixed_cell((row, col));
                        if new_cell == None { return false; }
                        // println!("Run out of possible values for this cell! Now on row {} and col {}", new_cell.unwrap().0, new_cell.unwrap().1);
                        self.solve(new_cell.unwrap().0, new_cell.unwrap().1);
                    }
                    if self.is_valid(&possible_value, &row, &col) {
                        self.grid[row][col].value = Some(possible_value);
                        break;
                    } else {
                        possible_value += 1;
                        // println!("New possible_value: {}", possible_value);
                        self.grid[row][col].value = None;
                    }
                }
            }
        }
        true
    }
}

fn main() {
    // Board config -> Board state (Vec<Vec<Cell>> -> Stored in Sudoku Board
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

    let mut board: Board = Board::new(board_config);
    board.display();
    let solvable = board.solve(0, 0);
    if !solvable {
        println!("This puzzle is not solvable.");
    }
    board.display();
    
    // Testing row/col/box checking functions
    // println!("{}", board.val_in_row(&3, &0));
    // println!("{}", board.val_in_row(&9, &1));
    // println!("{}", board.val_in_col(&5, &0));
    // println!("{}", board.val_in_col(&1, &3));
    // println!("{}", board.val_in_box(&4, &2, &5));
    // println!("{}", board.val_in_box(&1, &4, &7));

}
