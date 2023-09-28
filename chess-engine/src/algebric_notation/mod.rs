mod file;
mod rank;

pub fn from_string(string: &str) -> Result<u8, String> {
    if string.len() != 2 {
        return Err(format!("Invalid Algebric Notation \"{}\"", string));
    }
    let mut chars = string.chars();
    let file = file::from_char(chars.next().unwrap())?;
    let rank = rank::from_char(chars.next().unwrap())?;
    Ok((rank << 4) | (file & 0x7))
}

pub fn to_string(square: u8) -> String {
    let file = file::to_char(square);
    let rank = rank::to_char(square);
    format!("{}{}", file, rank)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[rustfmt::skip]
    const ALGEBRIC_NOTATION: [&str; 64] = [
        "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",   
        "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
        "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
        "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
        "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
        "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
        "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
        "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
    ];

    #[rustfmt::skip]
    const BOARD_128: [u8; 64] = [
        0,   01,  02,  03,  04,  05,  06,  07,
        16,  17,  18,  19,  20,  21,  22,  23,
        32,  33,  34,  35,  36,  37,  38,  39,
        48,  49,  50,  51,  52,  53,  54,  55,
        64,  65,  66,  67,  68,  69,  70,  71,
        80,  81,  82,  83,  84,  85,  86,  87,
        96,  97,  98,  99,  100, 101, 102, 103,
        112, 113, 114, 115, 116, 117, 118, 119,
    ];

    #[test]
    fn algebraic_notation() {
        for (&i, &square) in BOARD_128.iter().zip(ALGEBRIC_NOTATION.iter()) {
            assert_eq!(
                from_string(square).unwrap(),
                i,
                "Failed to parse {}",
                square
            );

            assert_eq!(to_string(i), square, "Failed to convert {}", i);
        }
    }

    #[test]
    fn invalid_algebraic_notation() {
        assert!(from_string("a").is_err());
        assert!(from_string("a9").is_err());
        assert!(from_string("i1").is_err());
        assert!(from_string("a0").is_err());
        assert!(from_string("a1a").is_err());
    }
}
