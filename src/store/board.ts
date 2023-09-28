import { createSlice } from "@reduxjs/toolkit";

interface BoardState {
  flipBoard: boolean;
  showLabel: boolean;
  showLegalMove: boolean;
}

const initialState: BoardState = {
  flipBoard: false,
  showLabel: true,
  showLegalMove: true,
};

const boardSlice = createSlice({
  name: "board",
  initialState,
  reducers: {
    toggleFlipBoard: (state: BoardState) => {
      state.flipBoard = !state.flipBoard;
    },
    toggleShowLabel: (state: BoardState) => {
      state.showLabel = !state.showLabel;
    },
    toggleShowLegalMove: (state: BoardState) => {
      state.showLegalMove = !state.showLegalMove;
    },
  },
});

export const { toggleFlipBoard, toggleShowLabel, toggleShowLegalMove } =
  boardSlice.actions;
export default boardSlice.reducer;
