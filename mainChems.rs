use rand::Rng;
use std::io;

const BOARD_SIZE: usize = 6;

const PIECE_KING: u8 = 1;
const PIECE_QUEEN: u8 = 2;
const PIECE_BISHOP: u8 = 3;
const PIECE_KNIGHT: u8 = 4;
const PIECE_PAWN: u8 = 5;

const PLAYER_1: u8 = 1;
const PLAYER_2: u8 = 2;

struct GameState {
    board: [[u8; BOARD_SIZE]; BOARD_SIZE],
    current_player: u8,
}

impl GameState {
    fn new() -> GameState {
        let mut game = GameState {
            board: [[0; BOARD_SIZE]; BOARD_SIZE],
            current_player: PLAYER_1,
        };
        game.init_board();
        game
    }

    fn init_board(&mut self) {
        // Place the pieces on the board
        self.board[0][0] = PIECE_KING;
        self.board[0][1] = PIECE_QUEEN;
        self.board[0][2] = PIECE_BISHOP;
        self.board[0][3] = PIECE_KNIGHT;
        self.board[0][4] = PIECE_BISHOP;
        self.board[0][5] = PIECE_QUEEN;

        for i in 0..BOARD_SIZE {
            self.board[1][i] = PIECE_PAWN;
            self.board[BOARD_SIZE - 2][i] = PIECE_PAWN;
        }

        self.board[BOARD_SIZE - 1][0] = PIECE_KING;
        self.board[BOARD_SIZE - 1][1] = PIECE_QUEEN;
        self.board[BOARD_SIZE - 1][2] = PIECE_BISHOP;
        self.board[BOARD_SIZE - 3][3] = PIECE_KNIGHT;
        self.board[BOARD_SIZE - 1][4] = PIECE_BISHOP;
        self.board[BOARD_SIZE - 1][5] = PIECE_QUEEN;
    }

    fn print_board(&self) {
        println!("  a b c d e f ");
        for i in 0..BOARD_SIZE {
            print!("{} ", i + 1);
            for j in 0..BOARD_SIZE {
                let piece = match self.board[i][j] {
                    0 => " ",
                    PIECE_KING => "K",
                    PIECE_QUEEN => "Q",
                    PIECE_BISHOP => "B",
                    PIECE_KNIGHT => "N",
                    PIECE_PAWN => "P",
                    _ => panic!("Invalid piece"),
                };
                print!("{} ", piece);
            }
            println!("{}", i + 1);
        }
        println!("  a b c d e f ");
    }

    fn roll_dice(&self) -> u8 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1, 7)
    }

    fn roll_for_piece(&self) -> u8 {
        let piece = loop {
            let piece = self.roll_dice();
            if self.has_piece(piece) {
                break piece;
            }
        };
        piece
    }

    fn has_piece(&self, piece: u8) -> bool {
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if self.board[i][j] == piece {
                    return true;
                }
            }
        }
        false
    }
