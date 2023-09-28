import { useSelector } from "react-redux";
import { RootState } from "./store";
import Toolbar from "@mui/material/Toolbar";
import Board from "./components/Board";
import Menu from "./components/Menu";
import FenEditor from "./components/FenEditor";
import Footer from "./components/Footer";

function App() {
  const isDarkMode = useSelector((state: RootState) => state.theme.isDarkMode);
  const background = isDarkMode ? "bg-gray-800" : "bg-gray-200";
  return (
    <div
      className={`h-screen w-screen flex flex-col items-center ${background} t-0 l-0`}
    >
      <span className="h-16 w-full bg-blue-500 top-0 left-0 absolute" />
      <div
        className={`flex flex-col items-center h-full ${background} w-board`}
      >
        <Toolbar className="w-full bg-blue-500 top-0 left-0 absolute">
          <h1 className="text-2xl text-white">Chess</h1>
          <span className="flex-grow" />
          <Menu />
        </Toolbar>
        <Board />
        <FenEditor />
        <Footer />
      </div>
    </div>
  );
}

export default App;
