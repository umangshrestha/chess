import themeReducer from "./theme";
import boardReducer from "./board";
import { configureStore } from "@reduxjs/toolkit";

const store = configureStore({
  reducer: {
    theme: themeReducer,
    board: boardReducer,
  },
});

export default store;
export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
