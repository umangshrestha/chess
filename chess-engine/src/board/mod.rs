mod king_position;
mod utils;
use std::ops::{Index, IndexMut};

use crate::piece::Piece;
use crate::{castling::CastlingTrait, fen_trait::FenParser, piece::PieceTrait};
use king_position::{KingPosition, KingPositionTrait};
use utils::abs;
use utils::is_offboard;

const BOARD_SIZE: usize = 128;
const ROOK_OFFSET: [usize; 2] = [16, 1];
const KNIGHT_OFFSET: [usize; 4] = [14, 18, 31, 33];
const BISHOP_OFFSET: [usize; 2] = [15, 17];
const KING_OFFSET: [usize; 4] = [17, 16, 15, 1];

/*
In 0x88:
* the first 8 rows are used to store the board,
* the last 8 rows are used to store the pieces that have been captured.

    a   b   c   d   e   f   g   h
  +---------------------------------+
8 | 112 113 114 115 116 117 118 119 | 120 121 122 123 124 125 126 127
7 | 96  97  98  99  100 101 102 103 | 104 105 106 107 108 109 110 111
6 | 80  81  82  83  84  85  86  87  | 88  89  90  91  92  93  94  95
5 | 64  65  66  67  68  69  70  71  | 72  73  74  75  76  77  78  79
4 | 48  49  50  51  52  53  54  55  | 56  57  58  59  60  61  62  63
3 | 32  33  34  35  36  37  38  39  | 40  41  42  43  44  45  46  47
2 | 16  17  18  19  20  21  22  23  | 24  25  26  27  28  29  30  31
1 | 0   1   2   3   4   5   6   7   | 8   9   10  11  12  13  14  15
  +---------------------------------+
    a   b   c   d   e   f   g   h
*/

#[rustfmt::skip]
const BOARD: [char; BOARD_SIZE] = [
    'R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    'P', 'P', 'P', 'P', 'P', 'P', 'P', 'P', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    'p', 'p', 'p', 'p', 'p', 'p', 'p', 'p', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
    'r', 'n', 'b', 'q', 'k', 'b', 'n', 'r', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
];

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Board {
    board: [Piece; BOARD_SIZE],
    king_position: KingPosition,
}

impl Index<usize> for Board {
    type Output = Piece;

    fn index(&self, index: usize) -> &Self::Output {
        &self.board[index]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.board[index]
    }
}

impl Board {
    /// This function is used to display the board in the terminal
    /// It is used for debugging purposes
    #[allow(dead_code)]
    pub fn display(&self) {
        for i in 0..8 {
            let i = 7 - i;
            print!("{} ", i + 1);
            for j in 0..8 {
                print!("{} ", self.board[i * 16 + j]);
            }
            println!();
        }
        println!("  a b c d e f g h\n");
    }
}

impl FenParser for Board {
    fn parse_fen(&mut self, value: &str) -> Result<(), String> {
        // fen starts with rank 8
        let mut rank = 7;
        let mut file = 0;
        self.king_position = KingPosition::default();
        let mut board = [Piece::Empty; BOARD_SIZE];

        for ch in value.chars() {
            if ch == '/' {
                if rank == 0 {
                    return Err("Extra rank found".to_string());
                }
                rank -= 1;
                file = 0;
                continue;
            }

            if ch.is_ascii_digit() {
                if ch == '0' || ch == '9' {
                    return Err(format!("Invalid Character \"{}\" in Board", ch));
                }
                file += ch.to_digit(10).unwrap() as usize;
                continue;
            }
            if file >= 8 {
                return Err(format!(
                    "File out of range at rank \"{}\", \"{}\"",
                    rank, value
                ));
            }

            let pos = rank * 16 + file;
            board[pos] = Piece::from_char(ch)?;
            if board[pos].is_king() {
                self.king_position.set_king_position(pos, ch == 'K');
            }
            file += 1;
        }
        if rank != 0 {
            return Err(format!("Expected 8 Ranks, Found {}", 7 - rank));
        }

        self.board = board;
        Ok(())
    }

    fn to_fen(&self) -> String {
        const O: u8 = b'0';
        let mut board_str = String::new();
        for rank in (0..8).rev() {
            let mut count = 0;
            for file in 0..8 {
                let pos = rank * 16 + file;
                match self[pos] {
                    Piece::Empty => count += 1,
                    piece => {
                        if count != 0 {
                            board_str.push(char::from(O + count as u8));
                            count = 0;
                        }
                        board_str.push(char::from(piece));
                    }
                }
            }
            if count != 0 {
                board_str.push(char::from(O + count as u8));
            }
            if rank != 0 {
                board_str.push('/');
            }
        }
        board_str
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut king_position = KingPosition::default();
        king_position.set_king_position(116, false);
        king_position.set_king_position(4, true);
        let mut board = [Piece::Empty; BOARD_SIZE];
        for (i, ch) in BOARD.iter().enumerate() {
            board[i] = Piece::from(*ch);
        }

        Board {
            board,
            king_position,
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_fen())
    }
}

impl Board {
    pub fn is_check(&self, white_king: bool) -> bool {
        self.is_attacked(
            self.king_position.get_king_position(white_king),
            !white_king,
        )
    }

    pub fn is_insufficient_material(&self) -> bool {
        let mut white_bishop = false;
        let mut black_bishop = false;
        let mut white_knight = false;
        let mut black_knight = false;
        let mut white_pieces = 0;
        let mut black_pieces = 0;
        for i in 0..BOARD_SIZE {
            match self[i] {
                Piece::WhiteBishop => white_bishop = true,
                Piece::BlackBishop => black_bishop = true,
                Piece::WhiteKnight => white_knight = true,
                Piece::BlackKnight => black_knight = true,
                Piece::WhitePawn | Piece::WhiteRook | Piece::WhiteQueen | Piece::WhiteKing => {
                    white_pieces += 1
                }
                Piece::BlackPawn | Piece::BlackRook | Piece::BlackQueen | Piece::BlackKing => {
                    black_pieces += 1
                }
                _ => {}
            }
        }
        white_pieces == 0 && black_pieces == 0
            || white_pieces == 1 && black_pieces == 0 && white_knight
            || white_pieces == 0 && black_pieces == 1 && black_knight
            || white_pieces == 1 && black_pieces == 1 && white_knight && black_knight
            || white_pieces == 1 && black_pieces == 1 && white_bishop && black_bishop
            || white_pieces == 2 && black_pieces == 1 && white_bishop && white_knight
            || white_pieces == 1 && black_pieces == 2 && black_bishop && black_knight
    }

    /// This function doesn't check if move is valid, it just moves the piece
    pub fn r#move(&mut self, from: usize, to: usize) {
        let piece = self.board[from];
        if !self[to].is_empty() {
            // add the piece to the captured pieces
            self.board[to + 8] = self[to];
        }
        self.board[to] = piece;
        if piece.is_king() {
            self.king_position.set_king_position(to, piece.is_white());
        }
        self.board[from] = Piece::Empty;
    }

    pub fn is_legal_knight_move(&self, from: usize, to: usize) -> bool {
        if self.board[from].is_knight() {
            let diff = abs(from, to);
            return KNIGHT_OFFSET.contains(&diff);
        }
        false
    }

    pub fn is_legal_bishop_move(&self, from: usize, to: usize) -> bool {
        if self.board[from].is_bishop() {
            return BISHOP_OFFSET
                .iter()
                .any(|&offset| self.check_if_not_blocked(from, to, offset));
        }
        false
    }

    pub fn is_legal_rook_move(&self, from: usize, to: usize) -> bool {
        if self.board[from].is_rook() {
            return ROOK_OFFSET
                .iter()
                .any(|&offset| self.check_if_not_blocked(from, to, offset));
        }
        false
    }

    pub fn is_legal_pawn_move(&self, from: usize, to: usize, en_passant: Option<u8>) -> bool {
        match self.board[from] {
            Piece::WhitePawn => {
                if to < from {
                    return false;
                }
                match to - from {
                    // when pawn moves one square forward
                    16 => self.board[to].is_empty(),
                    // when pawn moves two squares forward it should be in the second rank
                    32 => self.board[to].is_empty() && (16..24).contains(&from),
                    // when pawn captures there should be a piece in the destination square
                    15 | 17 => {
                        self.board[to].is_black()
                            || (en_passant.is_some() && en_passant.unwrap() as usize == to)
                    }
                    _ => false,
                }
            }
            Piece::BlackPawn => {
                if to > from {
                    return false;
                }
                match from - to {
                    16 => self.board[to].is_empty(),
                    32 => self.board[to].is_empty() && (96..104).contains(&from),
                    15 | 17 => {
                        self.board[to].is_white()
                            || (en_passant.is_some() && en_passant.unwrap() as usize == to)
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }

    pub fn is_legal_king_move(&self, from: usize, to: usize, castling: &dyn CastlingTrait) -> bool {
        if self.board[from].is_king() {
            let diff = abs(from, to);
            if KING_OFFSET.contains(&diff) {
                return !self.is_attacked(to, !self.board[from].is_white());
            }

            if diff == 2 && self.board[to].is_empty() {
                if self.board[from].is_white() && from == 4 {
                    if self.board[5].is_empty() && to == 6 && castling.can_white_king_castle() {
                        return !self.is_attacked(5, false) && !self.is_attacked(6, false);
                    }
                    if self.board[3].is_empty()
                        && to == 2
                        && self.board[1].is_empty()
                        && castling.can_white_queen_castle()
                    {
                        return !self.is_attacked(3, false) && !self.is_attacked(2, false);
                    }
                }

                if self.board[from].is_black() && from == 116 {
                    if self.board[117].is_empty() && to == 118 && castling.can_black_king_castle() {
                        return !self.is_attacked(117, true) && !self.is_attacked(118, true);
                    }
                    if self.board[115].is_empty()
                        && to == 114
                        && self.board[113].is_empty()
                        && castling.can_black_queen_castle()
                    {
                        return !self.is_attacked(115, true) && !self.is_attacked(114, true);
                    }
                }
            }
        }
        false
    }

    pub fn is_legal_queen_move(&self, from: usize, to: usize) -> bool {
        if self.board[from].is_queen() {
            return BISHOP_OFFSET
                .iter()
                .chain(ROOK_OFFSET.iter())
                .any(|&offset| self.check_if_not_blocked(from, to, offset));
        }
        false
    }
}

impl Board {
    fn is_piece_at(&self, pos: usize, piece: Piece) -> bool {
        !is_offboard(pos) && self[pos] == piece
    }

    fn check_if_not_blocked(&self, from: usize, to: usize, offset: usize) -> bool {
        if abs(from, to) % offset != 0 {
            return false;
        }
        if from > to {
            self.move_till_offset(to, from, offset)
        } else {
            self.move_till_offset(from, to, offset)
        }
    }

    fn move_till_offset(&self, start: usize, end: usize, offset: usize) -> bool {
        for i in (start + offset..end).step_by(offset) {
            if !self.is_piece_at(i, Piece::Empty) {
                return false;
            }
        }
        true
    }

    fn is_attacked(&self, pos: usize, attacked_by: bool) -> bool {
        if attacked_by {
            let is_attacked = [15, 17].into_iter().any(|offset| {
                pos.checked_sub(offset).is_some()
                    && self.is_piece_at(pos - offset, Piece::WhitePawn)
            });
            if is_attacked {
                return true;
            }
        } else if self.is_piece_at(pos + 15, Piece::BlackPawn)
            || self.is_piece_at(pos + 17, Piece::BlackPawn)
        {
            return true;
        }

        let enemy_knight = if attacked_by {
            Piece::WhiteKnight
        } else {
            Piece::BlackKnight
        };

        for &offset in KNIGHT_OFFSET.iter() {
            if self.is_piece_at(pos + offset, enemy_knight)
                || (pos.checked_sub(offset).is_some()
                    && self.is_piece_at(pos - offset, enemy_knight))
            {
                return true;
            }
        }

        let enemy_queen = if attacked_by {
            Piece::WhiteQueen
        } else {
            Piece::BlackQueen
        };
        let enemy_bishop = if attacked_by {
            Piece::WhiteBishop
        } else {
            Piece::BlackBishop
        };

        for &offset in BISHOP_OFFSET.iter() {
            let mut t_pos = pos + offset;
            while !utils::is_offboard(t_pos) {
                if self.is_piece_at(t_pos, enemy_bishop) || self.is_piece_at(t_pos, enemy_queen) {
                    return true;
                }
                if !self.is_piece_at(t_pos, Piece::Empty) {
                    break;
                }
                t_pos += offset;
            }
            t_pos = pos;
            while let Some(new_pos) = t_pos.checked_sub(offset) {
                t_pos = new_pos;
                if utils::is_offboard(t_pos) {
                    break;
                }
                if self.is_piece_at(t_pos, enemy_bishop) || self.is_piece_at(t_pos, enemy_queen) {
                    return true;
                }
                if !self.is_piece_at(t_pos, Piece::Empty) {
                    break;
                }
            }
        }

        let enemy_king = if attacked_by {
            Piece::WhiteKing
        } else {
            Piece::BlackKing
        };

        for &offset in KING_OFFSET.iter() {
            if self.is_piece_at(pos + offset, enemy_king)
                || (pos.checked_sub(offset).is_some() && self.is_piece_at(pos - offset, enemy_king))
            {
                return true;
            }
        }

        let enemy_rook = if attacked_by {
            Piece::WhiteRook
        } else {
            Piece::BlackRook
        };

        for &offset in ROOK_OFFSET.iter() {
            let mut t_pos = pos + offset;
            while !utils::is_offboard(t_pos) {
                if self.is_piece_at(t_pos, enemy_rook) || self.is_piece_at(t_pos, enemy_queen) {
                    return true;
                }
                if !self.is_piece_at(t_pos, Piece::Empty) {
                    break;
                }
                t_pos += offset;
            }
            t_pos = pos;
            while let Some(new_pos) = t_pos.checked_sub(offset) {
                t_pos = new_pos;
                if utils::is_offboard(t_pos) {
                    break;
                }
                if self.is_piece_at(t_pos, enemy_rook) || self.is_piece_at(t_pos, enemy_queen) {
                    return true;
                }
                if !self.is_piece_at(t_pos, Piece::Empty) {
                    break;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_board() {
        let board = Board::default();
        assert_eq!(
            board.to_fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
        );
    }

    #[test]
    fn test_board_parse_fen() {
        let mut board = Board::default();
        board
            .parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR")
            .unwrap();
        assert_eq!(
            board.to_fen(),
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"
        );
    }

    #[test]
    fn test_is_check() {
        let check_fens = vec![
            "8/4r3/8/4n3/8/3K4/8/8", //white king is in check by black knight
            "8/8/8/4n3/8/3K4/8/8",   //white king is in check
            "rnbqkbnr/pppppppp/8/8/8/8/PPPpPPPP/RNBQKBNR", //white king is in check
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPpPP/RNBQKBNR", //white king is in check
        ];
        let mut board = Board::default();
        for fen in check_fens {
            board.parse_fen(fen).unwrap();
            assert_eq!(board.is_check(true), true, "FEN: {}", fen);
        }
    }

    #[test]
    fn test_is_not_check() {
        let check_fens = vec!["rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"];
        let mut board = Board::default();
        for fen in check_fens {
            board.parse_fen(fen).unwrap();
            assert_eq!(board.is_check(true), false);
        }
    }
}
