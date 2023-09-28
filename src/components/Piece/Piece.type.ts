export type PieceType =
  | "P" // White Pawn
  | "R" // White Rook
  | "N" // White Knight
  | "B" // White Bishop
  | "Q" // White Queen
  | "K" // White King
  | "p" // Black Pawn
  | "r" // Black Rook
  | "n" // Black Knight
  | "b" // Black Bishop
  | "q" // Black Queen
  | "k"; // Black King

export type WhitePieceType = Extract<PieceType, Uppercase<PieceType>>;
export type BlackPieceType = Extract<PieceType, Lowercase<PieceType>>;
