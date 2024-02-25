use std::{fmt, io};

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty(usize),
    O,
    X,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match *self {
            Cell::Empty(index) => format!("[{index}]"),
            Cell::O => String::from(" O "),
            Cell::X => String::from(" X "),
        };

        write!(f, "{val}")
    }
}

const WIN_CASES: [[usize; 3]; 8] = [
    [0, 3, 6], // 1st column
    [1, 4, 7], // 2nd column
    [2, 5, 8], // 3rd column
    [0, 1, 2], // 1st row
    [3, 4, 5], // 2nd row
    [6, 7, 8], // 3rd row
    [0, 4, 8], // main diagonal
    [2, 4, 6], // secondary diagonal
];

type Board = [Cell; 9];

#[derive(PartialEq)]
enum GameResult {
    PlayerWon(usize),
    Draw,
}

#[derive(PartialEq)]
enum GameState {
    NotStarted,
    Ongoing,
    Finished(GameResult),
}

pub struct Game {
    board: Board,
    players: [Player; 2],
    current_player: usize,
    game_state: GameState,
}

impl Game {
    pub fn new(player1: Player, player2: Player) -> Game {
        let mut game = Game {
            board: [Cell::Empty(0); 9],
            players: [player1, player2],
            current_player: 0,
            game_state: GameState::NotStarted,
        };

        for i in 0..9 {
            game.board[i] = Cell::Empty(i + 1); // Put numbers 1..=9 into Empty cells. They'll
                                                // serve as cell positions
        }

        game
    }

    pub fn start(&mut self) {
        if self.game_state == GameState::NotStarted {
            self.game_state = GameState::Ongoing;

            while self.game_state == GameState::Ongoing {
                self.take_turn();
                self.check_if_over();

                self.current_player = if self.current_player == 0 { 1 } else { 0 };
            }

            self.announce_result();
        }
    }

    fn take_turn(&mut self) {
        let current_player = &self.players[self.current_player];

        clear_screen();
        self.print_board();

        let board_index = loop {
            let board_index = current_player.get_move(&self.board);
            if self.is_valid_move(board_index - 1) {
                break board_index - 1;
            } else {
                println!("'{board_index}' is not a valid board move!");
            }
        };

        self.current_player_make_move(board_index);
    }

    fn current_player_make_move(&mut self, board_index: usize) {
        self.board[board_index] = if self.current_player == 0 {
            Cell::O
        } else {
            Cell::X
        };
    }

    fn is_valid_move(&self, board_index: usize) -> bool {
        board_index < 9
            && match self.board[board_index] {
                Cell::Empty(_) => true,
                _ => false,
            }
    }

    fn check_if_over(&mut self) {
        let b = &self.board;

        for indices in &WIN_CASES {
            let (c1, c2, c3) = (&b[indices[0]], &b[indices[1]], &b[indices[2]]);
            if c1 == c2 && c1 == c3 {
                let winner = match c1 {
                    &Cell::O => 0,
                    &Cell::X => 1,
                    _ => continue, // false alarm - it's row/col/diag of empty cells
                };

                self.game_state = GameState::Finished(GameResult::PlayerWon(winner));
                return;
            }
        }

        let board_is_full = !self
            .board
            .iter()
            .any(|&cell| matches!(cell, Cell::Empty(_)));

        if board_is_full {
            self.game_state = GameState::Finished(GameResult::Draw);
        }
    }

    fn announce_result(&self) {
        clear_screen();
        self.print_board();

        match self.game_state {
            GameState::Finished(GameResult::Draw) => println!("It's a draw!"),
            GameState::Finished(GameResult::PlayerWon(player)) =>{
                if let Player::Human(name) = &self.players[player] {
                    println!("{} won!", name);
                }

            },
            _ => ()
        }
    }

    fn print_board(&self) {
        println!("     |     |");
        println!(
            " {} | {} | {} ",
            self.board[0], self.board[1], self.board[2]
        );
        println!("     |     |");

        println!("-----+-----+-----");

        println!("     |     |");
        println!(
            " {} | {} | {} ",
            self.board[3], self.board[4], self.board[5]
        );
        println!("     |     |");

        println!("-----+-----+-----");

        println!("     |     |");
        println!(
            " {} | {} | {} ",
            self.board[6], self.board[7], self.board[8]
        );
        println!("     |     |");
    }
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

pub enum Player {
    Human(String),
    CPU,
}

impl Player {
    fn get_move(&self, board: &Board) -> usize {
        match self {
            Self::Human(name) => Self::get_human_move(&name),
            Self::CPU => Self::get_computer_move(board),
        }
    }

    fn get_human_move(name: &str) -> usize {
        println!("Your move, {name}! Enter a number: ");

        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();

            break match buffer.trim().parse() {
                Ok(number) => number,
                Err(_) => {
                    println!("\nYour input is not a positive number! Try again:");
                    continue;
                }
            };
        }
    }

    fn get_computer_move(board: &Board) -> usize {
        1
    }
}
