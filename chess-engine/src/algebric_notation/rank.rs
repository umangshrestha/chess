const RANK_START: u8 = b'1';

fn is_valid(file: u8) -> bool {
    (RANK_START..=RANK_START + 7).contains(&file)
}

pub fn to_char(square: u8) -> char {
    let rank = square >> 4;
    (rank + RANK_START) as char
}

pub fn from_char(ch: char) -> Result<u8, String> {
    let rank = ch as u8;
    if !is_valid(rank) {
        return Err(format!("Invalid Rank: \"{}\"", ch));
    }
    Ok(rank - RANK_START)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_valid_rank() {
        for rank in 0..8 {
            assert_eq!(super::is_valid(RANK_START + rank), true);
        }
    }

    #[test]
    fn test_invalid_rank() {
        assert_eq!(super::is_valid(RANK_START + 9), false);
        assert_eq!(super::is_valid(RANK_START - 1), false);
    }
}
