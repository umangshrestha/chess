import { FC } from "react";
import { useChessContext } from "../context/useChessContext";

const GameStatus: FC = () => {
  const { gameStatus } = useChessContext();
  return <span className="text-white text-2xl font-bold">{gameStatus}</span>;
};

export default GameStatus;
