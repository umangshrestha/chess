pub trait CastlingTrait {
    fn can_white_king_castle(&self) -> bool;
    fn can_white_queen_castle(&self) -> bool;
    fn can_black_king_castle(&self) -> bool;
    fn can_black_queen_castle(&self) -> bool;

    fn set_white_king_castling(&mut self);
    fn set_white_queen_castling(&mut self);
    fn set_black_king_castling(&mut self);
    fn set_black_queen_castling(&mut self);

    fn reset_white_king_castling(&mut self);
    fn reset_white_queen_castling(&mut self);
    fn reset_black_king_castling(&mut self);
    fn reset_black_queen_castling(&mut self);
}
