import { FC } from "react";
import { LabelProps } from "./Label.types";

const Row = ["a", "b", "c", "d", "e", "f", "g", "h"];

export const RowLabel: FC<LabelProps> = ({ flipBoard, color, opacity }) => {
  const className = `flex justify-around ${
    flipBoard ? "flex-row-reverse" : "flex-row"
  } m-1 w-boardWithLabel ${color} font-bold ${opacity}`;
  return (
    <div className={className}>
      {Row.map((item, index) => (
        <div key={index}>{item}</div>
      ))}
    </div>
  );
};
