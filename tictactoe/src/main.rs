use std::fmt;
use std::io;

#[derive(Copy, Clone, Debug, PartialEq)]
enum BoardState {
    X = 0,
    O = 1,
    Blank = 2,
}

#[derive(PartialEq)]
enum GameState {
    InProgress,
    Winner(BoardState),
    Draw
}

impl fmt::Display for BoardState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self {
            BoardState::X => 'X',
            BoardState::O => 'O',
            BoardState::Blank => '.'
        };

        write!(f,"{}", symbol)
    }
}

struct Board {
    grid: [[BoardState; 3]; 3]
}

impl Board {
    fn new() -> Self {
        Self { grid: [[BoardState::Blank; 3]; 3]}
    }
    fn draw(&self) {
        println!("  0 1 2");
        for (i, row) in self.grid.iter().enumerate() {
            print!("{} ", i);

            for cell in row {
                print!("{} ", cell);
            }
            println!();
        }
    }
     
    fn check_game_state(&self) -> GameState {

        for player in [BoardState::X, BoardState::O] {
            // Check rows
            if self.grid.iter().any(|row| row.iter().all(|&cell| cell == player)) {
                return GameState::Winner(player);
            }

            // check cols
            if (0..3).any(|col_idx| {
                self.grid.iter().map(|row| &row[col_idx]).all(|&cell| cell == player) 
            }) {
                return GameState::Winner(player);
            }
            
            // check diag
            if (0..3).all(|i| self.grid[i][i] == player) {
                return GameState::Winner(player);
            }

            // check off diag
            if (0..3).all(|i| self.grid[i][2-i] == player) {
                return GameState::Winner(player);
            }
        }

        // check if board space still available
        if self.grid.iter().flatten().any(|&cell| cell == BoardState::Blank) {
            return GameState::InProgress;
        }

        GameState::Draw
    }

    fn make_move(&mut self, row: usize, col: usize, player: BoardState) -> Result<(), &str> {
        if row >= 3 || col >= 3 {
            return Err("Invalid coordinates");
        }

        if self.grid[row][col] != BoardState::Blank {
            return Err("This square is already filled");
        }
        
        self.grid[row][col] = player;
        Ok(())
    }
}   

fn next_turn(current: BoardState) -> BoardState {
    match current {
        BoardState::X => BoardState::O,
        BoardState::O => BoardState::X,
        _ => panic!("Invalid turn!"),
    }
}

fn parse_input(input_string: &str) -> Result<(usize, usize), &str> {
    let parts: Vec<&str> = input_string.split(',').collect();

    if parts.len() != 2 {
        return Err("Invalid number of inputs");
    }

    let row: usize = parts[0].trim().parse::<usize>().map_err(|_| "Invalid number")?;
    let col: usize = parts[1].trim().parse::<usize>().map_err(|_| "Invalid number")?;

    Ok((row,col))
}

fn main() {
    let mut board = Board::new();
                        
    let mut turn: BoardState = BoardState::O;
    let game_result = loop {
        // Check gameover
        let game_state: GameState = board.check_game_state();
        if game_state != GameState::InProgress {
            break game_state;
        }
        
        board.draw();
        println!("Player {turn}. Make your move by entering the \"row,col\" of the square you want to play");
        
        // collect input until valid
        let (row, col) = loop {
            let mut player_input = String::new();
            io::stdin().read_line(&mut player_input).expect("Couldn't read input.");

            match parse_input(&player_input) {
                Ok(coords) => break coords,
                Err(e) => println!("{}", e)
            }
        };
        let turn_result = board.make_move(row, col, turn);
        match turn_result {
            Ok(()) => turn = next_turn(turn),
            Err(e) => println!("{}", e)
        }
    };

    board.draw();
    match game_result {
        GameState::Draw => println!("Game ends in draw"),
        GameState::Winner(w) => println!("Player {} wins!", w),
        GameState::InProgress => panic!("Game should still be in progress")
    }

    
}

