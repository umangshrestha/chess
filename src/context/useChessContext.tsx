import { useContext } from "react";
import { ChessContext } from "./ChessContext";

export function useChessContext() {
  const context = useContext(ChessContext);
  if (!context) {
    throw new Error("useChessContext must be used within a ChessProvider");
  }
  return context;
}
