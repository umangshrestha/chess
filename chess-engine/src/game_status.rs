#[derive(Debug, PartialEq)]
pub enum GameStatus {
    InProgress,
    Draw,
    WhiteWon,
    BlackWon,
}

impl ToString for GameStatus {
    fn to_string(&self) -> String {
        match self {
            GameStatus::InProgress => "In Progress".to_string(),
            GameStatus::Draw => "Game Over (Draw)".to_string(),
            GameStatus::WhiteWon => "Game Over (White Won)".to_string(),
            GameStatus::BlackWon => "Game Over (Black Won)".to_string(),
        }
    }
}
