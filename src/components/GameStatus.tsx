import { FC } from "react";
import { useChessContext } from "../context/useChessContext";
import { RootState } from "../store";
import { useSelector } from "react-redux";

const GameStatus: FC = () => {
  const { gameStatus } = useChessContext();
  const isDarkMode = useSelector((state: RootState) => state.theme.isDarkMode);
  const className = `text-2xl font-bold ${
    isDarkMode ? "text-white" : "text-black"
  }`;
  return <span className={className}>{gameStatus}</span>;
};

export default GameStatus;
