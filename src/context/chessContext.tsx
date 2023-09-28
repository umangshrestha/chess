import { createContext, useContext } from "react";
import useChessEngine, { BoardType } from "../hooks/useChessEngine";

interface ChessContextType {
  board: BoardType;
  error: string | undefined;
  fen: string;
  setFen: (fen: string) => void;
  canDrag: (from: string) => boolean;
  canDrop: (from: string, to: string) => boolean;
  move: (from: string, to: string) => void;
}

const ChessContext = createContext<ChessContextType | undefined>(undefined);

export function useChessContext() {
  const context = useContext(ChessContext);
  if (!context) {
    throw new Error("useChessContext must be used within a ChessProvider");
  }
  return context;
}

export const ChessProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const chessData = useChessEngine();

  return (
    <ChessContext.Provider value={chessData}>{children}</ChessContext.Provider>
  );
};
