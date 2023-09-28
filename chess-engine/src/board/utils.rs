/// Check if pos is offboard in 0x88 board representation
pub fn is_offboard(pos: usize) -> bool {
    pos & 0x88 != 0
}

pub fn abs(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abs() {
        assert_eq!(abs(1, 2), 1);
        assert_eq!(abs(2, 1), 1);
        assert_eq!(abs(0, 0), 0);
    }
}
