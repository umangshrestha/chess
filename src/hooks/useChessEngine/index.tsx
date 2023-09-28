import { useState } from "react";
import chessEngine, { getBoard } from "./chessEngine";

export type BoardType = ReturnType<typeof getBoard>;

export default function useChessEngine() {
  const [board, setBoard] = useState(getBoard());
  const [error, setError] = useState<string | undefined>(undefined);
  const [fen, setFen] = useState(chessEngine.getFen());

  return {
    board,
    error,
    fen,
    setFen: (fen: string) => {
      setFen(fen);
      setError(chessEngine.setBoard(fen));
      setBoard(getBoard());
    },
    canDrag: (from: string) => chessEngine.canDrag(from),
    canDrop: (from: string, to: string) => chessEngine.canDrop(from, to),
    move: (from: string, to: string) => {
      chessEngine.move(from, to);
      const newBoard = getBoard();
      setBoard([...newBoard]);
      setFen((_) => chessEngine.getFen());
    },
  };
}
