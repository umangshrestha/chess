import init, { ChessEngine } from "chess-engine";

await init().catch((err) => console.error(err));
const chessEngine = new ChessEngine();

interface Board {
  piece: string;
  pos: string;
  isLightSquare: boolean;
}

export const getBoard = (): Board[] => {
  return chessEngine
    .getBoard()
    .split("")
    .map((piece, i) => {
      const [row, col] = [i >> 3, i & 7];
      const colName = String.fromCharCode(97 + col);
      const pos = `${colName}${8 - row}`;
      const isLightSquare = (row + col) % 2 === 0;
      return {
        piece,
        pos,
        isLightSquare,
      };
    });
};
export default chessEngine;
