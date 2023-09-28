import { useDrop } from "react-dnd";
import { FC, ReactNode } from "react";
import Overlay from "./Overlay";
import { useSelector } from "react-redux";
import { RootState } from "../store";
import { BoardThemeType, boardThemes } from "./Theme";
import { useChessContext } from "../context/useChessContext";

interface DropProps {
  pos: string;
  children: ReactNode;
  isLightSquare: boolean;
}

const Square: FC<DropProps> = ({ pos, children, isLightSquare }) => {
  const { canDrop: canDropFn, move } = useChessContext();
  const boardTheme = useSelector((state: RootState) => state.theme.board);
  const showLegalMove = useSelector(
    (state: RootState) => state.board.showLegalMove,
  );
  const { light, dark } = boardThemes[boardTheme as BoardThemeType];
  const backgroundColor = isLightSquare ? light : dark;

  const [{ isOver, canDrop }, drop] = useDrop(() => ({
    accept: "piece",
    drop: (item: { pos: string }) => move(item.pos, pos),
    canDrop: (item: { pos: string }) => canDropFn(item.pos, pos),
    collect: (monitor) => ({
      isOver: !!monitor.isOver(),
      canDrop: !!monitor.canDrop(),
    }),
  }));

  return (
    <div ref={drop} className={`relative ${backgroundColor}`}>
      {children}
      {showLegalMove && (
        <>
          {isOver && !canDrop && <Overlay color="red" />}
          {!isOver && canDrop && <Overlay color="yellow" />}
          {isOver && canDrop && <Overlay color="green" />}
        </>
      )}
    </div>
  );
};

export default Square;
