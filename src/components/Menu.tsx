import { FC, useState } from "react";
import { useSelector, useDispatch } from "react-redux";
import { RootState } from "../store";
import {
  toggleShowLegalMove,
  toggleFlipBoard,
  toggleShowLabel,
} from "../store/board";
import { toggleDarkMode } from "../store/theme";

import { FaGear } from "react-icons/fa6";
import Tooltip from "@mui/material/Tooltip";
import IconButton from "@mui/material/IconButton";
import Checkbox from "@mui/material/Checkbox";
import Dialog from "@mui/material/Dialog";
import DialogTitle from "@mui/material/DialogTitle";
import List from "@mui/material/List";
import ListItem from "@mui/material/ListItem";
import ListItemText from "@mui/material/ListItemText";
import Divider from "@mui/material/Divider";
import SelectPieceIcon from "./SelectPieceIcon";

const Menu: FC = () => {
  const dispatch = useDispatch();
  const { showLegalMove, flipBoard, showLabel } = useSelector(
    (state: RootState) => state.board,
  );
  const isDarkMode = useSelector((state: RootState) => state.theme.isDarkMode);
  const [open, setOpen] = useState(false);

  return (
    <div>
      <Tooltip title="Settings" placement="bottom">
        <IconButton onClick={() => setOpen((o) => !o)}>
          <FaGear color="white" className="2xl" />
        </IconButton>
      </Tooltip>
      <Dialog open={open} onClose={() => setOpen(false)} maxWidth="lg">
        <DialogTitle>Settings</DialogTitle>
        <Divider />
        <List>
          <ListItem>
            <Checkbox
              checked={showLegalMove}
              onChange={() => dispatch(toggleShowLegalMove())}
            />
            <ListItemText primary="Show Legal Moves" />
          </ListItem>
          <ListItem>
            <Checkbox
              checked={flipBoard}
              onChange={() => dispatch(toggleFlipBoard())}
            />
            <ListItemText primary="Flip Board" />
          </ListItem>

          <ListItem>
            <Checkbox
              checked={showLabel}
              onChange={() => dispatch(toggleShowLabel())}
            />
            <ListItemText primary="Show Label" />
          </ListItem>
          <ListItem>
            <Checkbox
              checked={isDarkMode}
              onChange={() => dispatch(toggleDarkMode())}
            />
            <ListItemText primary="Dark Mode" />
          </ListItem>
          <ListItem>
            <SelectPieceIcon />
          </ListItem>
        </List>
      </Dialog>
    </div>
  );
};

export default Menu;
