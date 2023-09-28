mod castling_trait;
use crate::fen_trait::FenParser;
pub use castling_trait::CastlingTrait;

// Bit 1 = White King Side, Bit 2 = White Queen Side, Bit 3 = Black King Side, Bit 4 = Black Queen Side
const WHITE_KING_CASTLING: u8 = 1 << 0;
const WHITE_QUEEN_CASTLING: u8 = 1 << 1;
const BLACK_KING_CASTLING: u8 = 1 << 2;
const BLACK_QUEEN_CASTLING: u8 = 1 << 3;

#[derive(Debug)]
pub struct Castling(pub u8);

impl CastlingTrait for Castling {
    fn can_white_king_castle(&self) -> bool {
        self.0 & WHITE_KING_CASTLING != 0
    }

    fn can_white_queen_castle(&self) -> bool {
        self.0 & WHITE_QUEEN_CASTLING != 0
    }

    fn can_black_king_castle(&self) -> bool {
        self.0 & BLACK_KING_CASTLING != 0
    }

    fn can_black_queen_castle(&self) -> bool {
        self.0 & BLACK_QUEEN_CASTLING != 0
    }

    fn set_white_king_castling(&mut self) {
        self.0 |= WHITE_KING_CASTLING;
    }

    fn set_white_queen_castling(&mut self) {
        self.0 |= WHITE_QUEEN_CASTLING;
    }

    fn set_black_king_castling(&mut self) {
        self.0 |= BLACK_KING_CASTLING;
    }

    fn set_black_queen_castling(&mut self) {
        self.0 |= BLACK_QUEEN_CASTLING;
    }

    fn reset_white_king_castling(&mut self) {
        self.0 &= !WHITE_KING_CASTLING;
    }

    fn reset_white_queen_castling(&mut self) {
        self.0 &= !WHITE_QUEEN_CASTLING;
    }

    fn reset_black_king_castling(&mut self) {
        self.0 &= !BLACK_KING_CASTLING;
    }

    fn reset_black_queen_castling(&mut self) {
        self.0 &= !BLACK_QUEEN_CASTLING;
    }
}

impl FenParser for Castling {
    fn to_fen(&self) -> String {
        if self.0 == 0 {
            return "-".to_string();
        }
        let mut castling_str = String::new();
        if self.can_white_king_castle() {
            castling_str.push('K');
        }
        if self.can_white_queen_castle() {
            castling_str.push('Q');
        }
        if self.can_black_king_castle() {
            castling_str.push('k');
        }
        if self.can_black_queen_castle() {
            castling_str.push('q');
        }
        castling_str
    }

    fn parse_fen(&mut self, castling_str: &str) -> Result<(), String> {
        if castling_str.len() > 4 {
            return Err(format!("Invalid Castling \"{}\"", castling_str));
        }
        self.0 = 0;
        if castling_str == "-" {
            return Ok(());
        }

        for ch in castling_str.chars() {
            match ch {
                'K' => self.0 |= WHITE_KING_CASTLING,
                'Q' => self.0 |= WHITE_QUEEN_CASTLING,
                'k' => self.0 |= BLACK_KING_CASTLING,
                'q' => self.0 |= BLACK_QUEEN_CASTLING,
                _ => return Err(format!("Invalid Character in Castling \"{}\"", ch)),
            }
        }
        Ok(())
    }
}

impl std::fmt::Display for Castling {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_fen())
    }
}

impl Default for Castling {
    fn default() -> Self {
        Self(
            WHITE_KING_CASTLING | WHITE_QUEEN_CASTLING | BLACK_KING_CASTLING | BLACK_QUEEN_CASTLING,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_castling() {
        let mut castling = Castling::default();
        castling.parse_fen("-").unwrap();
        assert_eq!(castling.can_white_king_castle(), false);
        assert_eq!(castling.can_white_queen_castle(), false);
        assert_eq!(castling.can_black_king_castle(), false);
        assert_eq!(castling.can_black_queen_castle(), false);
    }

    #[test]
    fn test_white_king_castling() {
        let mut castling = Castling::default();
        castling.parse_fen("K").unwrap();
        assert_eq!(castling.can_white_king_castle(), true);
        assert_eq!(castling.can_white_queen_castle(), false);
        assert_eq!(castling.can_black_king_castle(), false);
        assert_eq!(castling.can_black_queen_castle(), false);
    }

    #[test]
    fn test_white_queen_castling() {
        let mut castling = Castling::default();
        castling.parse_fen("Q").unwrap();
        assert_eq!(castling.can_white_king_castle(), false);
        assert_eq!(castling.can_white_queen_castle(), true);
        assert_eq!(castling.can_black_king_castle(), false);
        assert_eq!(castling.can_black_queen_castle(), false);
    }

    #[test]
    fn test_black_king_castling() {
        let mut castling = Castling::default();
        castling.parse_fen("k").unwrap();
        assert_eq!(castling.can_white_king_castle(), false);
        assert_eq!(castling.can_white_queen_castle(), false);
        assert_eq!(castling.can_black_king_castle(), true);
        assert_eq!(castling.can_black_queen_castle(), false);
    }

    #[test]
    fn test_black_queen_castling() {
        let mut castling = Castling::default();
        castling.parse_fen("q").unwrap();
        assert_eq!(castling.can_white_king_castle(), false);
        assert_eq!(castling.can_white_queen_castle(), false);
        assert_eq!(castling.can_black_king_castle(), false);
        assert_eq!(castling.can_black_queen_castle(), true);
    }

    #[test]
    fn test_all_castling() {
        let mut castling = Castling::default();
        assert_eq!(castling.can_white_king_castle(), true);
        assert_eq!(castling.can_white_queen_castle(), true);
        assert_eq!(castling.can_black_king_castle(), true);
        assert_eq!(castling.can_black_queen_castle(), true);

        castling.reset_white_king_castling();
        assert_eq!(castling.can_white_king_castle(), false);
        assert_eq!(castling.can_white_queen_castle(), true);
        assert_eq!(castling.can_black_king_castle(), true);
        assert_eq!(castling.can_black_queen_castle(), true);

        castling.reset_white_queen_castling();
        assert_eq!(castling.can_white_king_castle(), false);
        assert_eq!(castling.can_white_queen_castle(), false);
        assert_eq!(castling.can_black_king_castle(), true);
        assert_eq!(castling.can_black_queen_castle(), true);

        castling.reset_black_king_castling();
        assert_eq!(castling.can_white_king_castle(), false);
        assert_eq!(castling.can_white_queen_castle(), false);
        assert_eq!(castling.can_black_king_castle(), false);
        assert_eq!(castling.can_black_queen_castle(), true);

        castling.reset_black_queen_castling();
        assert_eq!(castling.can_white_king_castle(), false);
        assert_eq!(castling.can_white_queen_castle(), false);
        assert_eq!(castling.can_black_king_castle(), false);
        assert_eq!(castling.can_black_queen_castle(), false);
    }

    #[test]
    fn test_invalid_castling() {
        let mut castling = Castling::default();
        assert!(castling.parse_fen("KQkq").is_ok());
        assert!(castling.parse_fen("KQkqk").is_err());
    }
}
