import { useDrag } from "react-dnd";
import { FC } from "react";
import { useSelector } from "react-redux";
import { RootState } from "../store";
import Piece, { PieceType } from "./Piece";
import useChessEngine from "../hooks/useChessEngine";

interface DragProps {
  pos: string;
  piece: string;
}

export const DraggablePiece: FC<DragProps> = ({ pos, piece }) => {
  const icon = useSelector((state: RootState) => state.theme.icon);
  const { canDrag } = useChessEngine();

  const [{ isDragging }, drag] = useDrag(() => ({
    type: "piece",
    item: { pos },
    canDrag: () => canDrag(pos),
    collect: (monitor) => ({
      isDragging: !!monitor.isDragging(),
    }),
  }));

  if (piece === " ") return null;

  return (
    <div
      ref={drag}
      className="transition duration-500 ease-in-out"
      style={{
        cursor: canDrag(pos) ? "move" : "default",
        opacity: isDragging ? 0.2 : 0.9,
      }}
    >
      <Piece piece={piece as PieceType} icon={icon} />
    </div>
  );
};
