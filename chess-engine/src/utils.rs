/// converts a 0x88 position to a normal position 0-63
pub fn convert_postion_to_0x88(pos: usize) -> usize {
    (pos & 0x7) | ((pos & 0x38) << 1)
}
