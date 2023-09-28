import { FC } from "react";
import { AppDispatch, RootState } from "../store";
import { setBoardTheme } from "../store/theme";
import { boardThemes, BoardThemeType } from "./Theme/boardThemes";
import { useDispatch, useSelector } from "react-redux";
import { Tooltip } from "@mui/material";

const SelectBoardTheme: FC = () => {
  const currBoardTheme = useSelector((state: RootState) => state.theme.board);
  const dispatch: AppDispatch = useDispatch();
  const changeTheme = (theme: BoardThemeType) => dispatch(setBoardTheme(theme));

  return (
    <div className="grid grid-cols-3 m-4">
      {Object.keys(boardThemes).map((key) => {
        const { light, dark } = boardThemes[key as BoardThemeType];
        const buttonClass =
          key === currBoardTheme
            ? "flex m-2 p-1 border-2 border-blue-500"
            : "flex m-2 p-1 hover:scale-110 transition duration-500 ease-in-out bg-opacity-20 border-blue-500";

        return (
          <Tooltip
            title={
              <div className="text-center text-white text-2xl">
                {key.toUpperCase()}
              </div>
            }
            key={`tooltip-${key}`}
            placement="bottom"
          >
            <button
              key={key}
              onClick={() => changeTheme(key as BoardThemeType)}
              className={buttonClass}
            >
              <div className={`${light} h-themeSelector w-themeSelector`} />
              <div className={`${dark} h-themeSelector w-themeSelector`} />
            </button>
          </Tooltip>
        );
      })}
    </div>
  );
};

export default SelectBoardTheme;
