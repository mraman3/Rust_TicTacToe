// Aman Braich
// Jan 1st, 2025
// Rust - Tic Tac Toe
use clap::Parser;
use iroh::endpoint::Connection;
use iroh::{Endpoint, NodeAddr, NodeId, RelayMode, SecretKey};
use std::io::{self, Write};
use std::net::SocketAddr;

const EXAMPLE_ALPN: &[u8] = b"n0/tictactoe";

/// Represents a player in the game, either Player X or Player O.
///
/// The `Player` enum has two variants:
/// - `X`: Represents Player X, typically the First player.
/// - `O`: Represents Player O, typically the Second player.
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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
enum GameType {
    Client,
    Host,
}
#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    gametype: GameType,
    #[arg(short, long)]
    node_id: Option<NodeId>,
    #[clap(long, value_parser, num_args = 1.., value_delimiter = ' ')]
    addrs: Vec<SocketAddr>,
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
    connection: Connection,
}

impl Game {
    /// Creates and initializes a new Game instance.
    /// # Returns
    /// A `Game` object with the following initial state:
    /// - A 3x3 game board represented as a 2D array filled with `None` values, indicating an empty board.
    /// - The `current_player` set to `Player::X`, signifying that Player X will take the First turn.
    fn new(connection: Connection, current_player: Player) -> Game {
        Game {
            board: [[None, None, None], [None, None, None], [None, None, None]],
            current_player,
            connection,
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

    async fn get_coords_remote(&mut self) -> Vec<usize>{
        let mut recv = self.connection.accept_uni().await.unwrap();
        let raw_message = recv.read_to_end(100).await.unwrap();
        let message = String::from_utf8_lossy(&raw_message).to_string();

        let cleaned_input: Vec<usize> = message
            .trim()
            .split_whitespace()
            .filter_map(|x| x.parse().ok())
            .collect();

        cleaned_input
    }

    /// This method reads input from the user, expecting two numbers (the row and column)
    /// representing the desired position on the game board. It validates the input to
    /// ensure that it consists of two valid integers. If the input is invalid
    /// it will prompt the user again until the input is correct.
    /// # Returns
    /// A `cleaned_input Vec<usize>` containing two elements:
    /// - The First element is the row index.
    /// - The Second element is the column index.
    async fn get_coords(&mut self) -> Vec<usize> {
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
            if cleaned_input.len() == 2 {
                // sending coords
                let mut send = self.connection.open_uni().await.unwrap();
                let message = input.clone();
                send.write_all(message.as_bytes()).await.unwrap();
                send.finish().unwrap();
                // returning vector of coordinates
                break cleaned_input;
            }

            println!("Invalid input. Please enter two numbers (row and column).");
            // looping if invalid
            continue;
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
            println!("Invalid move: Out of bounds.");
            return Err("Out of bounds.");
        }

        if self.board[row][col].is_some() {
            println!("Invalid move: Cell already taken.");
            return Err("Cell already taken.");
        }

        self.board[row][col] = Some(self.current_player);
        self.print_board();
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

    // /// Checks if the game board is empty
    // /// This method iterates through all cells of the 3x3 game board to determine
    // /// if every cell is occupied by a None indicated an empty cell.
    // /// # Returns
    // /// - `true`: If every cell on the board is occupied by `None`.
    // /// - `false`: If there is at least one occupied cell (`Player::O` or `Player::X`).
    // fn is_empty(&self) -> bool {
    //     self.board
    //         .iter()
    //         .all(|row| row.iter().all(|&cell| cell.is_none()))
    // }
}

//noinspection DuplicatedCode
async fn host() {
    let secret_key = SecretKey::generate(rand::rngs::OsRng);
    let endpoint = Endpoint::builder()
        .secret_key(secret_key)
        .alpns(vec![EXAMPLE_ALPN.to_vec()])
        .relay_mode(RelayMode::Default)
        .bind()
        .await
        .unwrap();

    let me = endpoint.node_id();
    let node_addr = endpoint.node_addr().await.unwrap();
    let local_addrs = node_addr
        .direct_addresses
        .into_iter()
        .map(|addr| {
            let addr = addr.to_string();
            addr
        })
        .collect::<Vec<_>>()
        .join(" ");

    println!("\tcargo run -- client --node-id {me} --addrs \"{local_addrs}\"");

    let connection = endpoint
        .accept()
        .await
        .unwrap()
        .accept()
        .unwrap()
        .await
        .unwrap();

    let mut host_game = Game::new(connection, Player::X);
    // print initial empty board
    host_game.print_board();
    loop {
        // gets coordinates form user and sets it to row and col
        let mut coords: Vec<usize>;
        let mut row: usize = 3;
        let mut col: usize = 3;

        if host_game.current_player == Player::X {
            coords = host_game.get_coords().await;
            row = coords[0];
            col = coords[1];
        }

        if host_game.current_player == Player::O {
            coords =host_game.get_coords_remote().await;
            row = coords[0];
            col = coords[1];
        }

        // makes next move based of user coordinates
        match host_game.make_move(row, col) {
            // if accepted
            Ok(()) => {
                // Checks for winner/draw and updates board
                if let Some(winner) = host_game.check_winner() {

                    // prints win and ends game
                    println!("Player {:?} wins!", winner);
                    host_game.connection.close(Default::default(), &[]);
                    break;
                } else if host_game.check_winner() == None && host_game.is_full() {

                    // prints draw and ends game
                    println!("It's a draw!");
                    host_game.connection.close(Default::default(), &[]);
                    break;
                }
            }
            Err(_) => {}
        }
    }
}

async fn client(host: NodeId, addrs: Vec<SocketAddr>) {
    let secret_key = SecretKey::generate(rand::rngs::OsRng);
    println!("Secret Key: {}", secret_key);

    let endpoint = Endpoint::builder()
        .secret_key(secret_key)
        .alpns(vec![EXAMPLE_ALPN.to_vec()])
        .relay_mode(RelayMode::Default)
        .bind()
        .await
        .unwrap();

    let me = endpoint.node_id();
    println!("ME is: {}", me);

    let node_addr = NodeAddr::from_parts(
        host,
        Some("https://use1-1.relay.iroh.network./".parse().unwrap()),
        addrs,
    );
    let mut client_game = Game::new(endpoint.connect(node_addr, EXAMPLE_ALPN).await.unwrap(), Player::X);
    client_game.print_board();
    loop {
        // gets coordinates form user and sets it to row and col
        let mut coords: Vec<usize>;
        let mut row: usize = 3;
        let mut col: usize = 3;

        if client_game.current_player == Player::O {
            coords = client_game.get_coords().await;
            row = coords[0];
            col = coords[1];
        }

        if client_game.current_player == Player::X {
            coords = client_game.get_coords_remote().await;
            row = coords[0];
            col = coords[1];
        }

        // makes next move based of user coordinates
        match client_game.make_move(row, col) {
            // if accepted
            Ok(()) => {
                // Checks for winner/draw and updates board
                if let Some(winner) = client_game.check_winner() {
                    // prints win and ends game
                    println!("Player {:?} wins!", winner);
                    client_game.connection.close(Default::default(), &[]);
                    break;
                } else if client_game.check_winner() == None && client_game.is_full() {
                    // prints draw and ends game
                    println!("It's a draw!");
                    client_game.connection.close(Default::default(), &[]);
                    break;
                }
            }
            Err(_) => {}
        }
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
#[tokio::main]
async fn main() {
    let cli = Args::parse();
    match cli.gametype {
        GameType::Host => {
            host().await;
        }
        GameType::Client => {
            client(cli.node_id.unwrap(), cli.addrs).await;
        }
    }
}
