import { createSlice } from "@reduxjs/toolkit";
import type { PayloadAction } from "@reduxjs/toolkit";
import { BoardThemeType } from "../components/Theme";
import { PieceIconType } from "../components/Piece";

interface ThemeState {
  board: BoardThemeType;
  isDarkMode: boolean;
  icon: PieceIconType;
}

const initialState: ThemeState = {
  board: "green",
  isDarkMode: true,
  icon: "fa",
};

const themeSlice = createSlice({
  name: "theme",
  initialState,
  reducers: {
    setBoardTheme: (
      state: ThemeState,
      action: PayloadAction<BoardThemeType>,
    ) => {
      state.board = action.payload;
    },
    toggleDarkMode: (state: ThemeState) => {
      state.isDarkMode = !state.isDarkMode;
    },
    setIcon: (state: ThemeState, action: PayloadAction<PieceIconType>) => {
      state.icon = action.payload;
    },
  },
});

export const { setBoardTheme, toggleDarkMode, setIcon } = themeSlice.actions;
export default themeSlice.reducer;
