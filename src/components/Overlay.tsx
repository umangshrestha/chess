import { FC } from "react";

interface OverlayProps {
  color: string;
}

const Overlay: FC<OverlayProps> = ({ color }) => {
  const className = `absolute top-0 left-0 w-full h-full opacity-50 z-10`;
  return (
    <div
      className={className}
      style={{
        backgroundColor: color,
      }}
    />
  );
};

export default Overlay;
