mod piece_trait;
pub use piece_trait::PieceTrait;

/// Following the Standard Algebraic Notation (SAN), each piece is identified by a single letter taken from the standard English names
/// pawn = "P", knight = "N", bishop = "B", rook = "R", queen = "Q" and king = "K").
/// White pieces are designated using upper-case letters,  while black pieces use lowercase.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Piece {
    WhitePawn,
    WhiteKnight,
    WhiteBishop,
    WhiteRook,
    WhiteQueen,
    WhiteKing,
    BlackPawn,
    BlackKnight,
    BlackBishop,
    BlackRook,
    BlackQueen,
    BlackKing,
    Empty,
}

impl From<char> for Piece {
    fn from(ch: char) -> Self {
        Piece::from_char(ch).unwrap()
    }
}

impl From<Piece> for char {
    fn from(piece: Piece) -> Self {
        match piece {
            Piece::WhitePawn => 'P',
            Piece::WhiteKnight => 'N',
            Piece::WhiteBishop => 'B',
            Piece::WhiteRook => 'R',
            Piece::WhiteQueen => 'Q',
            Piece::WhiteKing => 'K',
            Piece::BlackPawn => 'p',
            Piece::BlackKnight => 'n',
            Piece::BlackBishop => 'b',
            Piece::BlackRook => 'r',
            Piece::BlackQueen => 'q',
            Piece::BlackKing => 'k',
            Piece::Empty => ' ',
        }
    }
}

impl Piece {
    pub fn from_char(ch: char) -> Result<Piece, String> {
        match ch {
            'P' => Ok(Piece::WhitePawn),
            'N' => Ok(Piece::WhiteKnight),
            'B' => Ok(Piece::WhiteBishop),
            'R' => Ok(Piece::WhiteRook),
            'Q' => Ok(Piece::WhiteQueen),
            'K' => Ok(Piece::WhiteKing),
            'p' => Ok(Piece::BlackPawn),
            'n' => Ok(Piece::BlackKnight),
            'b' => Ok(Piece::BlackBishop),
            'r' => Ok(Piece::BlackRook),
            'q' => Ok(Piece::BlackQueen),
            'k' => Ok(Piece::BlackKing),
            ' ' => Ok(Piece::Empty),
            _ => Err(format!("Invalid Chess Piece \"{}\"", ch)),
        }
    }
}

impl PieceTrait for Piece {
    fn is_empty(&self) -> bool {
        *self == Piece::Empty
    }

    fn is_white(&self) -> bool {
        matches!(
            *self,
            Piece::WhitePawn
                | Piece::WhiteKnight
                | Piece::WhiteBishop
                | Piece::WhiteRook
                | Piece::WhiteQueen
                | Piece::WhiteKing
        )
    }

    fn is_black(&self) -> bool {
        !self.is_white() && !self.is_empty()
    }

    fn is_pawn(&self) -> bool {
        matches!(*self, Piece::WhitePawn | Piece::BlackPawn)
    }

    fn is_knight(&self) -> bool {
        matches!(*self, Piece::WhiteKnight | Piece::BlackKnight)
    }

    fn is_bishop(&self) -> bool {
        matches!(*self, Piece::WhiteBishop | Piece::BlackBishop)
    }

    fn is_rook(&self) -> bool {
        matches!(*self, Piece::WhiteRook | Piece::BlackRook)
    }

    fn is_queen(&self) -> bool {
        matches!(*self, Piece::WhiteQueen | Piece::BlackQueen)
    }

    fn is_king(&self) -> bool {
        matches!(*self, Piece::WhiteKing | Piece::BlackKing)
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_to_char() {
        vec![
            (Piece::WhitePawn, 'P'),
            (Piece::WhiteKnight, 'N'),
            (Piece::WhiteBishop, 'B'),
            (Piece::WhiteRook, 'R'),
            (Piece::WhiteQueen, 'Q'),
            (Piece::WhiteKing, 'K'),
            (Piece::BlackPawn, 'p'),
            (Piece::BlackKnight, 'n'),
            (Piece::BlackBishop, 'b'),
            (Piece::BlackRook, 'r'),
            (Piece::BlackQueen, 'q'),
            (Piece::BlackKing, 'k'),
            (Piece::Empty, ' '),
        ]
        .into_iter()
        .for_each(|(piece, ch)| {
            assert_eq!(char::from(piece), ch);
        });
    }

    #[test]
    fn test_is_black() {
        vec![
            Piece::WhitePawn,
            Piece::WhiteKnight,
            Piece::WhiteBishop,
            Piece::WhiteRook,
            Piece::WhiteQueen,
            Piece::WhiteKing,
        ]
        .into_iter()
        .for_each(|piece| {
            assert!(!piece.is_black());
            assert!(piece.is_white());
        });

        vec![
            Piece::BlackPawn,
            Piece::BlackKnight,
            Piece::BlackBishop,
            Piece::BlackRook,
            Piece::BlackQueen,
            Piece::BlackKing,
        ]
        .into_iter()
        .for_each(|piece| {
            assert!(piece.is_black());
            assert!(!piece.is_white());
        });
    }

    #[test]
    fn test_is_proper_type() {
        assert!(Piece::WhitePawn.is_pawn());
        assert!(Piece::WhiteKnight.is_knight());
        assert!(Piece::WhiteBishop.is_bishop());
        assert!(Piece::WhiteRook.is_rook());
        assert!(Piece::WhiteQueen.is_queen());
        assert!(Piece::WhiteKing.is_king());
        assert!(Piece::BlackPawn.is_pawn());
        assert!(Piece::BlackKnight.is_knight());
        assert!(Piece::BlackBishop.is_bishop());
        assert!(Piece::BlackRook.is_rook());
        assert!(Piece::BlackQueen.is_queen());
        assert!(Piece::BlackKing.is_king());
        assert!(!Piece::Empty.is_pawn());
        assert!(!Piece::Empty.is_knight());
        assert!(!Piece::Empty.is_bishop());
        assert!(!Piece::Empty.is_rook());
        assert!(!Piece::Empty.is_queen());
        assert!(!Piece::Empty.is_king());
    }
}
