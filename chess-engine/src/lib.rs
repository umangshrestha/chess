mod algebric_notation;
mod board;
mod castling;
mod fen_trait;
mod game_status;
mod piece;
mod utils;
use board::Board;

use castling::{Castling, CastlingTrait};
use fen_trait::FenParser;
use game_status::GameStatus;
use piece::{Piece, PieceTrait};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = window)]
    fn alert(s: &str);
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct ChessEngine {
    /// The piece placement (from white's perspective). Each rank is described, starting with rank 8 and ending with rank 1;
    /// within each rank, the contents of each square are described from file "a" through file "h".
    board: Board,
    castling: Castling,
    /// The active player, in fen it is "w" or "b".
    is_white_turn: bool,
    /// The en passant square, if there is one, else "-".
    /// If a pawn has just made a two-square move, this is the position "behind" the pawn.
    en_passant: Option<u8>,
    /// The number of halfmoves since the last capture or pawn advance, used for the fifty-move rule.
    /// If value is 100, then the game is a draw due to the fifty-move rule.
    half_move_clock: u8,
    /// The number of full moves, it starts at 1 and is incremented after black's move.
    full_move_number: u16,
    game_status: GameStatus,
}

#[wasm_bindgen]
impl ChessEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Default::default()
    }

    #[wasm_bindgen(js_name = "getBoard")]
    pub fn get_board(&self) -> String {
        let mut board = [' '; 64];
        for i in 0..8 {
            for j in 0..8 {
                board[i * 8 + j] = char::from(self.board[(7 - i) * 16 + j]);
            }
        }
        board.iter().collect::<String>()
    }

    #[wasm_bindgen(js_name = "gameStatus")]
    pub fn game_status(&mut self) -> String {
        self.game_status.to_string().clone()
    }

    #[wasm_bindgen(js_name = "setBoard")]
    pub fn set_board(&mut self, fen_string: &str) -> JsValue {
        JsValue::from(self.parse_fen(fen_string).err())
    }

    #[wasm_bindgen(js_name = "canDrag")]
    pub fn can_drag(&self, from: &str) -> bool {
        let from = algebric_notation::from_string(from).unwrap();
        self.is_turn(from.into())
    }

    #[wasm_bindgen(js_name = "canDrop")]
    pub fn can_drop(&mut self, from: &str, to: &str) -> bool {
        let from = algebric_notation::from_string(from).unwrap().into();
        let to = algebric_notation::from_string(to).unwrap().into();
        self.test_move(from, to)
    }

    #[wasm_bindgen(js_name = "move")]
    pub fn r#move(&mut self, from: &str, to: &str) -> bool {
        let from = algebric_notation::from_string(from).unwrap().into();
        let to = algebric_notation::from_string(to).unwrap().into();
        let castling = self.castling.0;
        self.make_move(from, to);
        if self.board.is_check(self.is_white_turn) {
            self.castling.0 = castling;
            return false;
        }
        self.is_white_turn = !self.is_white_turn;
        self.game_status = self.get_status();
        true
    }

    #[wasm_bindgen(js_name = "getFen")]
    pub fn get_fen(&self) -> String {
        self.to_fen()
    }
}

impl FenParser for ChessEngine {
    fn parse_fen(&mut self, fen_string: &str) -> Result<(), String> {
        let mut parts = fen_string.split_whitespace();
        self.board
            .parse_fen(parts.next().expect("Board String not found"))?;

        let active_color = parts.next().unwrap_or("w");
        if active_color != "w" && active_color != "b" {
            return Err(format!(
                "Invalid Character \"{active_color}\" in active color"
            ));
        }
        self.is_white_turn = active_color == "w";
        self.castling.parse_fen(parts.next().unwrap_or("-"))?;
        self.en_passant = match parts.next().unwrap_or("-") {
            "-" => None,
            en_passant => Some(algebric_notation::from_string(en_passant)?),
        };
        let halfmove = parts.next().unwrap_or("0");
        self.half_move_clock = match halfmove.parse::<u8>() {
            Ok(halfmove_clock) => halfmove_clock,
            Err(_) => return Err(format!("Invalid Halfmove \"{halfmove}\"")),
        };

        let fullmove = parts.next().unwrap_or("1");
        self.full_move_number = match fullmove.parse::<u16>() {
            Ok(fullmove_number) => fullmove_number,
            Err(_) => return Err(format!("Invalid Fullmove \"{fullmove}\"")),
        };
        Ok(())
    }

    fn to_fen(&self) -> String {
        let mut fen = String::new();
        fen.push_str(&self.board.to_fen());
        fen.push(' ');
        fen.push(if self.is_white_turn { 'w' } else { 'b' });
        fen.push(' ');
        fen.push_str(&self.castling.to_fen());
        fen.push(' ');
        fen.push_str(&self.en_passant.map_or("-".to_string(), |en_passant| {
            algebric_notation::to_string(en_passant)
        }));
        fen.push(' ');
        fen.push_str(&self.half_move_clock.to_string());
        fen.push(' ');
        fen.push_str(&self.full_move_number.to_string());
        fen
    }
}

impl Default for ChessEngine {
    fn default() -> Self {
        Self {
            board: Board::default(),
            castling: Castling::default(),
            is_white_turn: true,
            en_passant: None,
            half_move_clock: 0,
            full_move_number: 1,
            game_status: GameStatus::InProgress,
        }
    }
}

impl ToString for ChessEngine {
    fn to_string(&self) -> String {
        self.to_fen()
    }
}

impl ChessEngine {
    fn is_turn(&self, pos: usize) -> bool {
        self.board[pos].is_white() == self.is_white_turn
    }

    fn get_status(&mut self) -> GameStatus {
        if self.board.is_check(true) {
            if self.is_stalement() {
                return GameStatus::BlackWon;
            }
            return GameStatus::InProgress;
        }
        if self.board.is_check(false) {
            if self.is_stalement() {
                return GameStatus::WhiteWon;
            }
            return GameStatus::InProgress;
        }
        if self.is_stalement() {
            return GameStatus::Draw;
        }
        GameStatus::InProgress
    }

    fn is_stalement(&mut self) -> bool {
        if self.board.is_insufficient_material() {
            return true;
        }
        for i in 0..64 {
            let from = utils::convert_postion_to_0x88(i);
            if !self.is_turn(from) {
                continue;
            }
            for j in 0..64 {
                let to = utils::convert_postion_to_0x88(j);
                if self.test_move(from, to) {
                    return false;
                }
            }
        }
        true
    }

    fn test_move(&mut self, from: usize, to: usize) -> bool {
        if !self.is_legal_move(from, to) {
            return false;
        }
        let to_elem = self.board[to];
        self.board[to] = self.board[from];
        self.board[from] = Piece::Empty;
        let is_check = self.board.is_check(self.is_white_turn);
        self.board[from] = self.board[to];
        self.board[to] = to_elem;
        !is_check
    }

    fn is_legal_move(&self, from: usize, to: usize) -> bool {
        from != to
            && self.is_turn(from)
            && (self.board[to].is_empty()
                || self.board[from].is_white() != self.board[to].is_white())
            && (self.board.is_legal_knight_move(from, to)
                || self.board.is_legal_bishop_move(from, to)
                || self.board.is_legal_rook_move(from, to)
                || self.board.is_legal_queen_move(from, to)
                || self.board.is_legal_king_move(from, to, &self.castling)
                || self.board.is_legal_pawn_move(from, to, self.en_passant))
    }

    fn make_move(&mut self, from: usize, to: usize) {
        let piece = self.board[from];
        let diff = (from as i8 - to as i8).abs();
        /* If no capture or pawn move, increment half move clock */
        if self.board[to].is_empty() {
            self.half_move_clock += 1;
        } else {
            self.half_move_clock = 0;
        }
        /* Update king position, castling and en passant */
        match piece {
            Piece::WhiteKing => {
                if diff == 2 {
                    if to == 2 {
                        self.board[0] = Piece::Empty;
                        self.board[3] = Piece::WhiteRook;
                    } else if to == 6 {
                        self.board[7] = Piece::Empty;
                        self.board[5] = Piece::WhiteRook;
                    }
                }
                self.castling.reset_white_king_castling();
                self.castling.reset_white_queen_castling();
            }
            Piece::BlackKing => {
                if diff == 2 {
                    if to == 118 {
                        self.board[119] = Piece::Empty;
                        self.board[117] = Piece::BlackRook;
                    } else if to == 112 {
                        self.board[112] = Piece::Empty;
                        self.board[115] = Piece::BlackRook;
                    }
                }
                self.castling.reset_black_king_castling();
                self.castling.reset_black_queen_castling();
            }
            Piece::BlackRook => {
                if from == 112 {
                    self.castling.reset_white_queen_castling()
                } else if from == 7 {
                    self.castling.reset_white_king_castling()
                }
            }
            Piece::WhiteRook => {
                if from == 0 {
                    self.castling.reset_black_queen_castling()
                } else if from == 7 {
                    self.castling.reset_black_king_castling()
                }
            }
            Piece::WhitePawn => {
                self.half_move_clock = 0;
                if let Some(pos) = self.en_passant {
                    if to == pos as usize && self.board[to - 16].is_black() {
                        self.board[to - 16] = Piece::Empty;
                    }
                    self.en_passant = None;
                } else if to - from == 32 {
                    self.en_passant = Some((from + 16) as u8);
                } else {
                    self.en_passant = None;
                }
            }
            Piece::BlackPawn => {
                self.half_move_clock = 0;
                if let Some(pos) = self.en_passant {
                    if to == pos as usize && self.board[to + 16].is_white() {
                        self.board[to + 16] = Piece::Empty;
                    }
                    self.en_passant = None;
                } else if from - to == 32 {
                    self.en_passant = Some((from - 16) as u8);
                } else {
                    self.en_passant = None;
                }
            }
            _ => {
                self.en_passant = None;
            }
        }

        self.board.r#move(from, to);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const FEN_STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    #[test]
    fn test_parse_fen_string() {
        let input = FEN_STARTING_POSITION;
        let mut chess_position = ChessEngine::new();
        chess_position.parse_fen(input).unwrap();
        assert_eq!(chess_position.is_white_turn, true);
        assert_eq!(chess_position.to_string(), input);
    }

    #[test]
    fn test_error_in_fen_string() {
        let input = "Jnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let mut chess_position = ChessEngine::new();
        let result: Result<(), String> = chess_position.parse_fen(input);
        assert_eq!(result.is_err(), true);
        assert!(result.unwrap_err().contains("Invalid Chess Piece \"J\""));

        let input = "rnbqkbnr/pppppppp/8/8/8/8";
        let result = chess_position.parse_fen(input);
        assert!(result.unwrap_err().contains("Expected 8 Ranks, Found 5"));

        let input = "rnbqkbnr/pppppppp/8/0/8/8/PPPPPPPP/RNBQKBNR";
        let result = chess_position.parse_fen(input);
        assert_eq!(result.is_err(), true);
        assert!(result
            .unwrap_err()
            .contains("Invalid Character \"0\" in Board"));

        let input = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR J KQkq - 0 1";
        let result = chess_position.parse_fen(input);
        assert_eq!(result.is_err(), true);
        assert!(result
            .unwrap_err()
            .contains("Invalid Character \"J\" in active color"));

        let input = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w JQkq - 0 1";
        let result = chess_position.parse_fen(input);
        assert_eq!(result.is_err(), true);
        assert!(result
            .unwrap_err()
            .contains("Invalid Character in Castling \"J\""));

        let input = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq JJ 0 1";
        let result = chess_position.parse_fen(input);
        assert_eq!(result.is_err(), true);
        assert!(result.unwrap_err().contains("Invalid File \"J\""));

        let input = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - JJ 1";
        let result = chess_position.parse_fen(input);
        assert_eq!(result.is_err(), true);
        assert!(result.unwrap_err().contains("Invalid Halfmove \"JJ\""));

        let input = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 JJ";
        let result = chess_position.parse_fen(input);
        assert_eq!(result.is_err(), true);
        assert!(result.unwrap_err().contains("Invalid Fullmove \"JJ\""));
    }

    #[test]
    fn test_can_enpassant() {
        let mut chess_position = ChessEngine::new();
        let arr: Vec<(&str, (&str, &str))> = vec![
            (
                "rnbqkbnr/pppp1ppp/8/3Pp3/8/8/PPPPPPPP/RNBQKBNR w KQkq e6 0 2",
                ("d5", "e6"),
                // "enpassant for black pawn",
            ),
            (
                "rnbqkbnr/pppp1ppp/8/8/3Pp3/8/PPP1PPPP/RNBQKBNR b KQkq d3 0 1",
                ("e4", "d3"),
                // "enpassant for white pawn",
            ),
        ];
        arr.into_iter().for_each(|(fen, (from, to))| {
            chess_position.set_board(fen);
            assert_eq!(
                chess_position.can_drop(from, to),
                true,
                "{}->{}| {}",
                from,
                to,
                fen
            );
        });
    }

    #[test]
    fn test_cannot_enpassant() {
        let mut chess_position = ChessEngine::new();
        let arr: Vec<(&str, (&str, &str))> = vec![
            (
                "rnbqkbnr/pppp1ppp/8/3Pp3/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 2",
                ("d5", "e6"),
                // "enpassant for black pawn when flag not set",
            ),
            (
                "rnbqkbnr/pppp1ppp/8/8/3Pp3/8/PPP1PPPP/RNBQKBNR b KQkq - 0 1",
                ("e4", "d3"),
                // "enpassant for white pawn when flag not set",
            ),
        ];
        arr.into_iter().for_each(|(fen, (from, to))| {
            chess_position.set_board(fen);
            assert_eq!(
                chess_position.can_drop(from, to),
                false,
                "{}->{}| {}",
                from,
                to,
                fen
            );
        });
    }

    #[test]
    fn test_valid_can_drop() {
        let mut chess_position = ChessEngine::new();
        let arr: Vec<(&str, (&str, &str))> = vec![
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                ("a2", "a3"),
                // "single white pawn move when square is empty",
            ),
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                ("h2", "h4"),
                // "double white pawn move when square is empty",
            ),
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1",
                ("a7", "a6"),
                // "single black pawn move when square is empty",
            ),
            (
                "8/8/8/8/8/8/8/R7 w - - 0 1",
                ("a1", "a8"),
                // "white rook move when square is empty (vertical)",
            ),
            (
                "8/8/8/8/8/8/8/R7 w - - 0 1",
                ("a1", "h1"),
                // "white rook move when square is empty (horizontal)",
            ),
            (
                "r7/8/8/8/8/8/8/8 b - - 0 1",
                ("a8", "a1"),
                // "black rook move when square is empty (vertical)",
            ),
            (
                "r7/8/8/8/8/8/8/8 b - - 0 1",
                ("a8", "h8"),
                // "black rook move when square is empty (horizontal)",
            ),
            (
                "8/8/8/8/8/8/8/B8 w - - 0 1",
                ("a1", "h8"),
                // "white bishop move when white square is empty (diagonal)",
            ),
            (
                "8/8/8/8/8/8/8/7B w - - 0 1",
                ("h1", "a8"),
                // "white bishop move when black square is empty (diagonal)",
            ),
            (
                "b7/8/8/8/8/8/8/8 b - - 0 1",
                ("a8", "h1"),
                // "black bishop move when white square is empty (diagonal)",
            ),
            (
                "7b/8/8/8/8/8/8/8 b - - 0 1",
                ("h8", "a1"),
                // "black bishop move when black square is empty (diagonal)",
            ),
        ];
        arr.into_iter().for_each(|(fen, (from, to))| {
            chess_position.set_board(fen);
            assert_eq!(
                chess_position.can_drop(from, to),
                true,
                "{}->{}| {}",
                from,
                to,
                fen
            );
        });
    }

    #[test]
    fn test_cannot_make_king_to_check() {
        let mut chess_position = ChessEngine::new();
        let arr: Vec<(&str, (&str, &str))> = vec![(
            "rnb1kbnr/pppp1ppp/8/4p3/4P2q/3P4/PPP2PPP/RNBQKBNR w KQkq - 1 1",
            ("f1", "f2"),
        )];
        arr.into_iter().for_each(|(fen, (from, to))| {
            chess_position.set_board(fen);
            assert_eq!(
                chess_position.can_drop(from, to),
                false,
                "{}->{}| {}",
                from,
                to,
                fen
            );
        });
    }
    #[test]
    fn test_cannot_drop() {
        let mut chess_position = ChessEngine::new();
        let arr: Vec<(&str, (&str, &str))> = vec![
            (
                "rnbqkbnr/ppppppp1/8/8/8/7p/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                ("h2", "h3"),
                // "single white pawn move when black pawn is occupying space",
            ),
            (
                "rnbqkbnr/pppppppp/8/8/7P/7P/PPPPPP1P/RNBQKBNR w KQkq - 0 1",
                ("h2", "h3"),
                // "single white pawn move when white pawn is occupying space",
            ),
            (
                "rnbqkbnr/pppppppp/8/8/7P/7P/PPPPPP1P/RNBQKBNR w KQkq - 0 1",
                ("h2", "h3"),
                // "single white pawn move when white pawn is occupying space",
            ),
            (
                "rnbqkbnr/ppppppp1/8/8/7p/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                ("h2", "h4"),
                // "double white pawn move when black pawn is occupying space",
            ),
            (
                "rnbqkbnr/pppppppp/8/8/7P/8/PPPPPP1P/RNBQKBNR w KQkq - 0 1",
                ("h2", "h4"),
                // "double white pawn move when white pawn is occupying space",
            ),
            (
                "rnbqkbnr/pppppppp/P7/8/8/8/1PPPPPPP/RNBQKBNR b KQkq - 0 1",
                ("a7", "a6"),
                // "single black pawn move when white pawn is occupying space",
            ),
            (
                "rnbqkbnr/pppppppp/p7/8/8/8/1PPPPPPP/RNBQKBNR b KQkq - 0 1",
                ("a7", "a6"),
                // "single black pawn move when black pawn is occupying space",
            ),
        ];
        arr.into_iter().for_each(|(fen, (from, to))| {
            chess_position.set_board(fen);
            assert_eq!(
                chess_position.can_drop(from, to),
                false,
                "{}->{}| {}",
                from,
                to,
                fen
            );
            assert_eq!(chess_position.get_status(), GameStatus::InProgress);
        });
    }

    #[test]
    fn test_can_castle() {
        let mut chess_position = ChessEngine::new();
        let arr: Vec<(&str, (&str, &str))> = vec![
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R w KQkq - 0 1",
                ("e1", "g1"),
                // "white king side castle",
            ),
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3K3 w KQkq - 0 1",
                ("e1", "c1"),
                // "white queen side castle",
            ),
            (
                "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1",
                ("e8", "g8"),
                // "black king side castle",
            ),
            (
                "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R b KQkq - 0 1",
                ("e8", "c8"),
                // "black queen side castle",
            ),
        ];
        arr.into_iter().for_each(|(fen, (from, to))| {
            chess_position.set_board(fen);
            assert_eq!(
                chess_position.can_drop(from, to),
                true,
                "{}->{}| {}",
                from,
                to,
                fen
            );
        });
    }

    #[test]
    fn test_one_castling_after_another() {
        let mut chess_position = ChessEngine::new();
        let fen = "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/R3K2R w KQkq - 0 1";
        chess_position.set_board(fen);
        assert_eq!(chess_position.can_drop("e1", "g1"), true);
        assert_eq!(chess_position.can_drop("e1", "c1"), true);
        chess_position.r#move("e1", "g1");
        assert_eq!(chess_position.can_drop("e8", "g8"), true);
        assert_eq!(chess_position.can_drop("e8", "c8"), true);
        chess_position.r#move("e8", "c8");
    }

    #[test]
    fn test_cannot_make_move_when_checkmate() {
        let mut chess_position = ChessEngine::new();
        let arr: Vec<(&str, (&str, &str), GameStatus)> = vec![
            (
                "rnbqk2r/pppp1Qpp/5n2/2b1p3/2B1P3/8/PPPP1PPP/RNB1K1NR b KQkq - 0 4",
                ("d7", "d6"),
                GameStatus::WhiteWon,
                // "moving pawn when scholar's mate",
            ),
            (
                "rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3",
                ("a1", "a2"),
                GameStatus::BlackWon,
                // "moving rook when fool's mate",
            ),
        ];
        arr.into_iter().for_each(|(fen, (from, to), status)| {
            chess_position.set_board(fen);
            assert_eq!(
                chess_position.can_drop(from, to),
                false,
                "{}->{}| {}",
                from,
                to,
                fen
            );
            assert_eq!(chess_position.get_status(), status);
        });
    }

    #[test]
    fn test_cannot_castle() {
        let mut chess_position = ChessEngine::new();
        let arr: Vec<(&str, (&str, &str))> = vec![
            (
                "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQq - 0 1",
                ("e8", "g8"),
                // "black king side castle when flag is not set",
            ),
            (
                "r3k2r/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R b KQk - 0 1",
                ("e8", "c8"),
                // "black queen side castle when flag is not set",
            ),
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R w Qkq - 0 1",
                ("e1", "g1"),
                // "white king side castle when flag is not set",
            ),
            (
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/R3K3 w Kkq - 0 1",
                ("e1", "c1"),
                // "white queen side castle",
            ),
        ];
        arr.into_iter().for_each(|(fen, (from, to))| {
            chess_position.set_board(fen);
            assert_eq!(
                chess_position.can_drop(from, to),
                false,
                "{}->{}| {}",
                from,
                to,
                fen
            );
        });
    }
}
