// Aman Braich
// Jan 1st, 2025
// Rust - Tic Tac Toe
use std::io::{self, Write};

/// Represents a player in the game, either Player X or Player O.
///
/// The `Player` enum has two variants:
/// - `X`: Represents Player X, typically the first player.
/// - `O`: Represents Player O, typically the second player.
///
/// The `Player` enum is derived with the following traits:
/// - `Copy`: Allows the enum to be copied rather than moved, enabling easy value duplication.
/// - `Clone`: Provides a way to explicitly duplicate the enum value.
/// - `PartialEq`: Allows comparison between two `Player` values using `==` and `!=`.
/// - `Debug`: Enables formatting for the `Player` type using the `{:?}` formatting
///    specifier for debugging purposes.
#[derive(Copy, Clone, PartialEq, Debug)]
enum Player {
    X,
    O,
}

impl Player {
    /// This method switches between `Player::X` and `Player::O`.
    /// - If the current player is `Player::X`, it returns `Player::O`.
    /// - If the current player is `Player::O`, it returns `Player::X`.
    fn other(self) -> Player {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

/// Represents the state of a Tic-Tac-Toe game.
/// # Fields
/// - `board`: A 3x3 grid represented as a 2D array of `Option<Player>`.
///   - Each cell can be:
///     - `Some(Player::X)` if occupied by Player X.
///     - `Some(Player::O)` if occupied by Player O.
///     - `None` if the cell is empty.
/// - `current_player`: The player whose turn it is to make a move.
///   - Can be `Player::X` or `Player::O`.
struct Game {
    board: [[Option<Player>; 3]; 3],
    current_player: Player,
}

impl Game {
    /// Creates and initializes a new Game instance.
    /// # Returns
    /// A `Game` object with the following initial state:
    /// - A 3x3 game board represented as a 2D array filled with `None` values, indicating an empty board.
    /// - The `current_player` set to `Player::X`, signifying that Player X will take the first turn.
    fn new() -> Game {
        Game {
            board: [[None, None, None], [None, None, None], [None, None, None]],
            current_player: Player::X,
        }
    }

    /// Prints the current state of the game board to the console.
    /// - `X` represents a cell occupied by Player X.
    /// - `O` represents a cell occupied by Player O.
    /// - `.` represents an empty cell.
    fn print_board(&self) {
        println!("Current board:");
        for row in &self.board {
            for &cell in row {
                match cell {
                    Some(Player::X) => print!(" X "),
                    Some(Player::O) => print!(" O "),
                    None => print!(" . "),
                }
            }
            println!();
        }
    }

    /// This method reads input from the user, expecting two numbers (the row and column)
    /// representing the desired position on the game board. It validates the input to
    /// ensure that it consists of two valid integers. If the input is invalid
    /// it will prompt the user again until the input is correct.
    /// # Returns
    /// A `cleaned_input Vec<usize>` containing two elements:
    /// - The first element is the row index.
    /// - The second element is the column index.
    fn get_coords(&mut self) -> Vec<usize> {
        loop {
            // input from user
            let mut input = String::new();
            // print prompt
            print!(
                "Player {:?}, enter your move (row col): ",
                self.current_player
            );

            // reading input from user
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();

            // parsing string from user to vector
            let cleaned_input: Vec<usize> = input
                .trim()
                .split_whitespace()
                .filter_map(|x| x.parse().ok())
                .collect();

            // Checking if vector contains both coordinates
            if cleaned_input.len() != 2 {
                println!("Invalid input. Please enter two numbers (row and column).");
                // looping if invalid
                continue;
            }
            // returning vector of coordinates
            break cleaned_input;
        }
    }

    /// Attempts to make a move on the game board at the specified position. Then updates the
    /// board with the player marker
    /// # Parameters
    /// - `row`: The row index where the move is to be made.
    /// - `col`: The column index where the move is to be made.
    /// # Returns
    /// - `Ok(())`: If the move is successful.
    /// - `Err(&str)`: If the move is invalid. Possible error messages:
    ///   - `"Invalid move: Out of bounds."` if the row or column index is out of range
    ///      (greater than or equal to 3).
    ///   - `"Invalid move: Cell already taken."` if the targeted cell is already occupied.
    fn make_move(&mut self, row: usize, col: usize) -> Result<(), &str> {
        if row >= 3 || col >= 3 {
            return Err("Invalid move: Out of bounds.");
        }

        if self.board[row][col].is_some() {
            return Err("Invalid move: Cell already taken.");
        }

        self.board[row][col] = Some(self.current_player);
        self.current_player = self.current_player.other();
        Ok(())
    }

    /// Checks if there is a winner in the game.
    /// This method examines the current game board to determine if either player
    /// has achieved a winning condition. A player wins if they have three of
    /// their markers (`X` or `O`) in a row, column, or diagonal line.
    /// # Returns
    /// - `Some(Player)`: If a player has won, returns the winning player (`Player::X` or `Player::O`).
    /// - `None`: If there is no winner, returns `None`.
    fn check_winner(&self) -> Option<Player> {
        for i in 0..3 {
            // Check rows
            if let Some(player) = self.board[i][0] {
                if self.board[i][1] == Some(player) && self.board[i][2] == Some(player) {
                    return Some(player);
                }
            }
            // Checks Columns
            if let Some(player) = self.board[0][i] {
                if self.board[1][i] == Some(player) && self.board[2][i] == Some(player) {
                    return Some(player);
                }
            }
        }

        // Check L-Diagonal
        if let Some(player) = self.board[0][0] {
            if self.board[1][1] == Some(player) && self.board[2][2] == Some(player) {
                return Some(player);
            }
        }
        // Checks R-Diagonal
        if let Some(player) = self.board[0][2] {
            if self.board[1][1] == Some(player) && self.board[2][0] == Some(player) {
                return Some(player);
            }
        }
        None
    }

    /// Checks if the game board is full
    /// This method iterates through all cells of the 3x3 game board to determine
    /// if every cell is occupied by a player.
    /// # Returns
    /// - `true`: If every cell on the board is occupied by either `Player::X` or `Player::O`.
    /// - `false`: If there is at least one empty cell (`None`).
    fn is_full(&self) -> bool {
        self.board
            .iter()
            .all(|row| row.iter().all(|&cell| cell.is_some()))
    }

    /// Checks if the game board is empty
    /// This method iterates through all cells of the 3x3 game board to determine
    /// if every cell is occupied by a None indicated an empty cell.
    /// # Returns
    /// - `true`: If every cell on the board is occupied by `None`.
    /// - `false`: If there is at least one occupied cell (`Player::O` or `Player::X`).
    fn is_empty(&self) -> bool {
        self.board
            .iter()
            .all(|row| row.iter().all(|&cell| cell.is_none()))
    }
}

/// The Main function of the Tic-Tac-Toe game. It will start a loop, get the coordinate input,
/// check for a winner, then end the game when a winner is found or a draw occurs.
/// This function performs the following tasks:
/// - Initializes the `Game::new()` and starts the game.
/// - Continuously calls `game.make_move(row, col)`.
/// - Then calls `game.check_winner()` and `game.is_full()` after each move
///   until the game ends via draw or by a win
/// - Then calls `game.print_board()` to keep the game updated after each move.
fn main() {
    // Initialize new game
    let mut game = Game::new();
    loop {
        // print initial empty board
        game.print_board();

        // gets coordinates form user and sets it to row and col
        let coords = game.get_coords();
        let row = coords[0];
        let col = coords[1];

        // makes next move based of user coordinates
        match game.make_move(row, col) {
            // if accepted
            Ok(()) => {
                // Checks for winner/draw and updates board
                if let Some(winner) = game.check_winner() {
                    game.print_board();

                    // prints win and ends game
                    println!("Player {:?} wins!", winner);
                    break;
                } else if game.check_winner() == None && game.is_full() {
                    game.print_board();

                    // prints draw and ends game
                    println!("It's a draw!");
                    break;
                }
            }
            Err(_) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game_initialization() {
        let game = Game::new();
        // Check that the board is empty
        assert!(game.is_empty());
        // Check that the starting player is X
        assert_eq!(game.current_player, Player::X);
    }

    #[test]
    fn test_make_valid_move() {
        let mut game = Game::new();
        //Make move check if result comes back true
        let result = game.make_move(1, 1);
        assert!(result.is_ok());
        assert_eq!(game.board[1][1], Some(Player::X));
        // Player changes after a valid move
        assert_eq!(game.current_player, Player::O);
    }

    #[test]
    fn test_make_invalid_move_out_of_bounds() {
        let mut game = Game::new();
        let result = game.make_move(3, 3); // Invalid position
        assert_eq!(result, Err("Invalid move: Out of bounds."));
    }

    #[test]
    fn test_make_invalid_move_cell_taken() {
        let mut game = Game::new();
        game.make_move(0, 0).unwrap(); // X plays
        let result = game.make_move(0, 0); // Attempt to play in the same cell
        assert_eq!(result, Err("Invalid move: Cell already taken."));
    }

    #[test]
    fn test_check_win_row() {
        let mut game = Game::new();
        //XXX
        //-0-
        //--0
        game.make_move(0, 0).unwrap(); // X
        game.make_move(1, 1).unwrap(); // O
        game.make_move(0, 1).unwrap(); // X
        game.make_move(2, 2).unwrap(); // O
        game.make_move(0, 2).unwrap(); // X wins

        assert_eq!(game.check_winner(), Some(Player::X));
    }

    #[test]
    fn test_check_win_column() {
        let mut game = Game::new();
        //X--
        //X0-
        //X-0
        game.make_move(0, 0).unwrap(); // X
        game.make_move(1, 1).unwrap(); // O
        game.make_move(1, 0).unwrap(); // X
        game.make_move(2, 2).unwrap(); // O
        game.make_move(2, 0).unwrap(); // X wins

        assert_eq!(game.check_winner(), Some(Player::X));
    }

    #[test]
    fn test_check_win_diagonal() {
        let mut game = Game::new();
        //X--
        //0X-
        //-0X
        game.make_move(0, 0).unwrap(); // X
        game.make_move(1, 0).unwrap(); // O
        game.make_move(1, 1).unwrap(); // X
        game.make_move(2, 1).unwrap(); // O
        game.make_move(2, 2).unwrap(); // X wins

        assert_eq!(game.check_winner(), Some(Player::X));
    }

    #[test]
    fn test_check_full_board() {
        let mut game = Game::new();
        //X0X
        //0X0
        //X0X
        game.make_move(0, 0).unwrap(); // X
        game.make_move(0, 1).unwrap(); // O
        game.make_move(0, 2).unwrap(); // X
        game.make_move(1, 0).unwrap(); // O
        game.make_move(1, 1).unwrap(); // X
        game.make_move(1, 2).unwrap(); // O
        game.make_move(2, 0).unwrap(); // X
        game.make_move(2, 1).unwrap(); // O
        game.make_move(2, 2).unwrap(); // X

        assert!(game.is_full());
    }

    #[test]
    fn test_check_empty_board() {
        let game = Game::new();
        //---
        //---
        //---
        assert!(game.is_empty());
    }

    #[test]
    fn test_draw() {
        //0X0
        //0XX
        //X0X
        let mut game = Game::new();
        game.make_move(0, 1).unwrap(); // X
        game.make_move(0, 0).unwrap(); // O
        game.make_move(1, 1).unwrap(); // X
        game.make_move(0, 2).unwrap(); // O
        game.make_move(1, 2).unwrap(); // X
        game.make_move(1, 0).unwrap(); // O
        game.make_move(2, 0).unwrap(); // X
        game.make_move(2, 1).unwrap(); // O
        game.make_move(2, 2).unwrap(); // X

        assert_eq!(game.check_winner(), None); // No winner, board is full
        assert!(game.is_full());
    }
}
