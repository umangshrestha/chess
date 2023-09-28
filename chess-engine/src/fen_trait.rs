pub trait FenParser {
    fn parse_fen(&mut self, fen_string: &str) -> Result<(), String>;

    fn to_fen(&self) -> String;
}

impl ToString for dyn FenParser {
    fn to_string(&self) -> String {
        self.to_fen()
    }
}
