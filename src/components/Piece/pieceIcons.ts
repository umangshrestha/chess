import {
  FaChessBishop,
  FaChessKing,
  FaChessKnight,
  FaChessPawn,
  FaChessQueen,
  FaChessRook,
} from "react-icons/fa";

import {
  GiChessBishop,
  GiChessKing,
  GiChessKnight,
  GiChessPawn,
  GiChessQueen,
  GiChessRook,
} from "react-icons/gi";

import {
  TbChessBishopFilled,
  TbChessKingFilled,
  TbChessKnightFilled,
  TbChessFilled as TbChessPawnFilled,
  TbChessQueenFilled,
  TbChessRookFilled,
} from "react-icons/tb";
import { WhitePieceType } from "./Piece.type";

type ChessIcons = {
  [key in WhitePieceType]: React.FC<any>;
};

const FaChessIcons: ChessIcons = {
  P: FaChessPawn,
  R: FaChessRook,
  N: FaChessKnight,
  B: FaChessBishop,
  Q: FaChessQueen,
  K: FaChessKing,
};

const GiChessIcons: ChessIcons = {
  P: GiChessPawn,
  R: GiChessRook,
  N: GiChessKnight,
  B: GiChessBishop,
  Q: GiChessQueen,
  K: GiChessKing,
};

const TbChessIcons: ChessIcons = {
  P: TbChessPawnFilled,
  R: TbChessRookFilled,
  N: TbChessKnightFilled,
  B: TbChessBishopFilled,
  Q: TbChessQueenFilled,
  K: TbChessKingFilled,
};

export const PieceLookupTable: {
  [key in PieceIconType]: ChessIcons;
} = {
  fa: FaChessIcons,
  gi: GiChessIcons,
  tb: TbChessIcons,
};

export const pieceIcons = ["fa", "gi", "tb"] as const;

export type PieceIconType = (typeof pieceIcons)[number];
