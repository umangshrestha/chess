import { useSelector } from "react-redux";
import { RootState } from "../store";
import MiniBoard from "../components/MiniBoard";
import SelectPieceIcon from "../components/SelectPieceIcon";
import SelectBoardTheme from "../components/SelectBoardTheme";

const Settings = () => {
  const darkMode = useSelector((state: RootState) => state.theme.darkMode);
  const textColor = darkMode ? "text-white" : "text-black";

  return (
    <div
      className={`flex flex-col justify-center items-center h-full w-full text-center text-4xl ${textColor}`}
    >
      <section>
        <h2>Preview</h2>
        <MiniBoard />
      </section>
      <section>
        <h2>Select Piece Icon</h2>
        <SelectPieceIcon />
      </section>
      <section>
        <h2>Select Board Theme</h2>
        <SelectBoardTheme />
      </section>
    </div>
  );
};

export default Settings;
