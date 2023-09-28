import useChessEngine from "../hooks/useChessEngine";
import { ChessContext } from "./ChessContext";

export const ChessProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const chessData = useChessEngine();

  return (
    <ChessContext.Provider value={chessData}>{children}</ChessContext.Provider>
  );
};
