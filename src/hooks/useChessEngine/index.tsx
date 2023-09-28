import { useState } from "react";
import chessEngine, { getBoard } from "./chessEngine";

export type BoardType = ReturnType<typeof getBoard>;

export default function useChessEngine() {
  // This hook is used in useChessContext()
  // Use useChessContext() instead of this hook for rerendering the component
  const [board, setBoard] = useState(getBoard());
  const [error, setError] = useState<string | undefined>(undefined);
  const [fen, setFen] = useState(chessEngine.getFen());
  const [gameStatus, setGameStatus] = useState(chessEngine.gameStatus());

  return {
    board,
    error,
    fen,
    setGameStatus,
    setFen: (fen: string) => {
      setFen(fen);
      setError(chessEngine.setBoard(fen));
      setBoard(getBoard());
      setGameStatus(chessEngine.gameStatus());
    },
    canDrag: (from: string) => chessEngine.canDrag(from),
    canDrop: (from: string, to: string) => chessEngine.canDrop(from, to),
    move: (from: string, to: string) => {
      chessEngine.move(from, to);
      const newBoard = getBoard();
      setBoard([...newBoard]);
      setFen(chessEngine.getFen());
      setGameStatus(chessEngine.gameStatus());
    },
    gameStatus,
  };
}
