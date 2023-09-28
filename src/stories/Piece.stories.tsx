import type { Meta } from "@storybook/react";
import Piece from "../components/Piece";

const meta = {
  title: "Chess/Piece",
  component: Piece,
  decorators: [
    (Story) => (
      <div
        style={{
          fontSize: "64px",
          backgroundColor: "green",
          height: "80px",
          width: "80px",
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
        }}
      >
        <Story />
      </div>
    ),
  ],
  args: {
    icon: "fa",
  },
  argTypes: {
    icon: {
      control: {
        type: "inline-radio",
        options: ["fa", "gi", "tb"],
      },
    },
    piece: {
      control: {
        type: "select",
        options: ["p", "n", "b", "r", "q", "k", "P", "N", "B", "R", "Q", "K"],
        lables: {
          p: "WhitePawn",
          n: "WhiteKnight",
          b: "WhiteBishop",
          r: "WhiteRook",
          q: "WhiteQueen",
          k: "WhiteKing",
          P: "BlackPawn",
          N: "BlackKnight",
          B: "BlackBishop",
          R: "BlackRook",
          Q: "BlackQueen",
          K: "BlackKing",
        },
      },
    },
  },
} as Meta;

export default meta;

export const Default = {
  args: {
    piece: "p",
  },
};
