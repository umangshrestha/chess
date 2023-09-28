import { FC } from "react";
import { RowLabel, ColLabel } from "../components/Label";
import { useSelector } from "react-redux";
import { RootState } from "../store";
import { LabelProps } from "../components/Label/Label.types";

const withLabel = <P extends Pick<LabelProps, "flipBoard">>(
  WrappedComponent: FC<P>,
) => {
  const WithLabel: FC<Omit<P, "flipBoard">> = (props) => {
    const { showLabel, flipBoard } = useSelector(
      (state: RootState) => state.board,
    );
    const { isDarkMode } = useSelector((state: RootState) => state.theme);
    const color = isDarkMode ? "text-white" : "text-black";
    const opacity = showLabel ? "opacity-100" : "opacity-0";
    const args = { flipBoard, opacity, color };
    return (
      <>
        <RowLabel {...args} />
        <div className="flex flex-row">
          <ColLabel {...args} />
          <WrappedComponent {...(props as P)} flipBoard={flipBoard} />
          <ColLabel {...args} />
        </div>
        <RowLabel {...args} />
      </>
    );
  };

  return WithLabel;
};

export default withLabel;
