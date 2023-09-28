import { createContext } from "react";
import { BoardType } from "../hooks/useChessEngine";

interface ChessContextType {
  board: BoardType;
  error: string | undefined;
  fen: string;
  gameStatus: string;
  setFen: (fen: string) => void;
  canDrag: (from: string) => boolean;
  canDrop: (from: string, to: string) => boolean;
  move: (from: string, to: string) => void;
  setGameStatus: (status: string) => void;
}

export const ChessContext = createContext<ChessContextType | undefined>(
  undefined,
);
