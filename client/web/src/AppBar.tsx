import { css } from "@emotion/react";
import {
  AppBar as MuiAppBar,
  Icon,
  IconButton,
  Toolbar,
  Typography,
} from "@material-ui/core";

import { useAppContext } from "./App";

export default function AppBar() {
  const app = useAppContext();
  return (
    <MuiAppBar
      color="inherit"
      position="fixed"
      css={(theme: any) => css`
        z-index: ${theme.zIndex.drawer + 1};
      `}
    >
      <Toolbar>
        <IconButton
          edge="start"
          color="inherit"
          onClick={app.openDrawer}
          css={(theme) =>
            css`
              margin-right: ${theme.spacing(2)};

              ${theme.breakpoints.up("md")} {
                display: none;
              }
            `
          }
        >
          <Icon>menu</Icon>
        </IconButton>
        <Typography
          variant="h6"
          component="div"
          css={css`
            flex-grow: 1;
          `}
        >
          Zenith
        </Typography>
      </Toolbar>
    </MuiAppBar>
  );
}
