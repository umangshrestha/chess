import { useDispatch, useSelector } from "react-redux";
import Piece from "./Piece";
import { AppDispatch, RootState } from "../store";
import { pieceIcons, PieceIconType } from "./Piece/pieceIcons";
import { setIcon } from "../store/theme";
import { Tooltip } from "@mui/material";

export const PieceIconNames: {
  [key in PieceIconType]: string;
} = {
  fa: "Font Awesome",
  gi: "Game Icons",
  tb: "Tabler Icons",
} as const;

const SelectPieceIcon = () => {
  const dispatch: AppDispatch = useDispatch();
  const currIcon = useSelector((state: RootState) => state.theme.icon);
  return (
    <div className="flex justify-center items-center">
      {pieceIcons.map((icon) => {
        const isSelected = icon === currIcon;
        const buttonClass = isSelected ? "border-blue-500" : "bg-opacity-20";

        return (
          <Tooltip
            title={
              <div className="text-center text-white text-2xl">
                {PieceIconNames[icon]}
              </div>
            }
            key={icon}
            placement="bottom"
          >
            <div
              key={icon}
              className={`flex justify-center ml-2 mr-2 p-1 bg-blue-500 ${buttonClass}`}
              onClick={() => dispatch(setIcon(icon as PieceIconType))}
            >
              <Piece key={icon} piece="K" icon={icon as PieceIconType} />
            </div>
          </Tooltip>
        );
      })}
    </div>
  );
};

export default SelectPieceIcon;
