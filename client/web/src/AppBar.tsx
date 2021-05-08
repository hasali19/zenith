import { css } from "@emotion/react";
import {
  Hidden,
  AppBar,
  Icon,
  IconButton,
  Toolbar,
  Typography,
} from "@material-ui/core";

export interface Props {
  onToggleDrawer: () => void;
}

export default function ({ onToggleDrawer }: Props) {
  return (
    <AppBar
      color="inherit"
      position="fixed"
      css={(theme: any) => css`
        z-index: ${theme.zIndex.drawer + 1};
      `}
    >
      <Toolbar>
        <Hidden mdUp implementation="css">
          <IconButton
            edge="start"
            color="inherit"
            onClick={onToggleDrawer}
            css={(theme) =>
              css`
                margin-right: ${theme.spacing(2)};
              `
            }
          >
            <Icon>menu</Icon>
          </IconButton>
        </Hidden>
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
    </AppBar>
  );
}
