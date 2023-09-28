import { FC, useMemo } from "react";
import { LabelProps } from "../Label";

import { DraggablePiece } from "../DraggablePiece";
import Square from "../Square";
import { useChessContext } from "../../context/useChessContext";

interface BoardProps extends Pick<LabelProps, "flipBoard"> {}

export const Board: FC<BoardProps> = ({ flipBoard }) => {
  const { board: data, error } = useChessContext();
  const board = flipBoard ? [...data].reverse() : data;

  const boardSquares = useMemo(
    () =>
      board.map(({ piece, pos, isLightSquare }) => (
        <Square pos={pos} key={`key-${pos}`} isLightSquare={isLightSquare}>
          <DraggablePiece pos={pos} piece={piece} />
        </Square>
      )),
    [board],
  );

  return (
    <>
      <div className="grid grid-cols-8 h-board w-board grid-gap-0 relative">
        {boardSquares}
        {error && (
          <div className="absolute top-0 left-0 w-full h-full flex justify-center items-center bg-black bg-opacity-60">
            <div className="text-red-500 text-2xl font-bold">{error}</div>
          </div>
        )}
      </div>
    </>
  );
};
