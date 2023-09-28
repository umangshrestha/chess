import { FC } from "react";
import { LabelProps } from "./Label.types";

const Col = ["8", "7", "6", "5", "4", "3", "2", "1"];

export const ColLabel: FC<LabelProps> = ({ flipBoard, opacity, color }) => {
  const className = `flex justify-around ${
    flipBoard ? "flex-col-reverse" : "flex-col"
  } h-boardWithLabel w-label m-1  text-center ${color} font-bold ${opacity}`;
  return (
    <div className={className}>
      {Col.map((item, index) => (
        <div key={index}>{item}</div>
      ))}
    </div>
  );
};
