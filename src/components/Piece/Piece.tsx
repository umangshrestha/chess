import { PieceType } from "./Piece.type";
import { PieceLookupTable, PieceIconType } from "./pieceIcons";
import { WhitePieceType } from "./Piece.type";
import { FC } from "react";

interface PieceProps {
  piece: PieceType;
  icon: PieceIconType;
}

export const Piece: FC<PieceProps> = ({ piece, icon }) => {
  const isWhite = piece === piece.toUpperCase();
  const color = isWhite ? "white" : "black";
  const Icon = PieceLookupTable[icon][piece.toUpperCase() as WhitePieceType];
  return <Icon color={color} className="text-6xl hover:scale-110" />;
};
