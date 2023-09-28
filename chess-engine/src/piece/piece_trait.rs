pub trait PieceTrait {
    fn is_empty(&self) -> bool;
    fn is_white(&self) -> bool;
    fn is_black(&self) -> bool;
    fn is_pawn(&self) -> bool;
    fn is_knight(&self) -> bool;
    fn is_bishop(&self) -> bool;
    fn is_rook(&self) -> bool;
    fn is_queen(&self) -> bool;
    fn is_king(&self) -> bool;
}
