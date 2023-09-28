const BLACK_KING_MASK: u16 = 0x00FF;
const WHITE_KING_MASK: u16 = 0xFF00;

// The first 0-7 bits are white king, and 8-15 position is for black king
pub type KingPosition = u16;

pub trait KingPositionTrait {
    fn get_king_position(&self, is_white: bool) -> usize;
    fn set_king_position(&mut self, pos: usize, is_white: bool);
}

impl KingPositionTrait for KingPosition {
    fn set_king_position(&mut self, pos: usize, is_white: bool) {
        let pos = pos as u16;
        if is_white {
            *self &= !WHITE_KING_MASK;
            *self |= pos << 8;
        } else {
            *self &= !BLACK_KING_MASK;
            *self |= pos;
        }
    }

    fn get_king_position(&self, is_white: bool) -> usize {
        let pos = if is_white {
            (self & WHITE_KING_MASK) >> 8
        } else {
            self & BLACK_KING_MASK
        };
        pos as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_king_position() {
        let mut king_position: KingPosition = 0;
        king_position.set_king_position(116, false);
        king_position.set_king_position(4, true);
        assert_eq!(king_position.get_king_position(false), 116);
        assert_eq!(king_position.get_king_position(true), 4);

        king_position.set_king_position(3, false);
        assert_eq!(king_position.get_king_position(false), 3);

        king_position.set_king_position(127, true);
        assert_eq!(king_position.get_king_position(true), 127);
    }
}
