import { css } from "@emotion/react";
import {
  AppBar as MuiAppBar,
  Icon,
  IconButton,
  Toolbar,
  Typography,
} from "@material-ui/core";

import { useAppContext } from "./App";

export interface Props {
  title?: string | boolean;
  transparent?: boolean;
}

export default function AppBar({ transparent, title }: Props) {
  const app = useAppContext();
  return (
    <MuiAppBar
      color={transparent ? "transparent" : "inherit"}
      position="fixed"
      elevation={transparent ? 0 : undefined}
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
        {title && (
          <Typography
            variant="h6"
            component="div"
            css={css`
              flex-grow: 1;
            `}
          >
            {title}
          </Typography>
        )}
      </Toolbar>
    </MuiAppBar>
  );
}
