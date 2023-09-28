import { useSelector } from "react-redux";
import { RootState } from "../store";
import { FC } from "react";
import { useChessContext } from "../context/useChessContext";

const FenEditor: FC = () => {
  const { fen, setFen } = useChessContext();
  const isDarkMode = useSelector((state: RootState) => state.theme.isDarkMode);

  return (
    <div
      className="flex flex-row m-4"
      style={{ color: isDarkMode ? "white" : "black" }}
    >
      <label className="text-2xl">FEN: </label>
      <input
        className="ml-2 p-1 rounded-md w-[33rem] border-2 border-blue-500"
        value={fen}
        style={{ backgroundColor: isDarkMode ? "#1f2937" : "#f3f4f6" }}
        onChange={(e) => setFen(e.target.value)}
      />
    </div>
  );
};

export default FenEditor;
