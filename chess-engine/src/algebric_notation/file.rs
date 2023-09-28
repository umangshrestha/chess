const FILE_START: u8 = b'a';

fn is_valid(file: u8) -> bool {
    (FILE_START..=FILE_START + 7).contains(&file)
}

pub fn to_char(square: u8) -> char {
    let file = square & 0x7;
    (file + FILE_START) as char
}

pub fn from_char(ch: char) -> Result<u8, String> {
    let file = ch as u8;
    if !is_valid(file) {
        return Err(format!("Invalid File \"{}\"", ch));
    }
    Ok(file - FILE_START)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_is_valid_rank() {
        for rank in 0..8 {
            assert_eq!(super::is_valid(FILE_START + rank), true);
        }
    }

    #[test]
    fn test_invalid_rank() {
        assert_eq!(super::is_valid(FILE_START + 9), false);
        assert_eq!(super::is_valid(FILE_START - 1), false);
    }
}
