import { css } from "@emotion/react";
import {
  AppBar,
  Icon,
  IconButton,
  Toolbar,
  Typography,
  useTheme,
} from "@material-ui/core";

export interface Props {
  onToggleDrawer: () => void;
}

export default function ({ onToggleDrawer }: Props) {
  const theme = useTheme();
  console.log(theme.breakpoints.up('md'));
  return (
    <AppBar
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
          onClick={onToggleDrawer}
          css={(theme) =>
            css`
                margin-right: ${theme.spacing(2)};

                ${theme.breakpoints.up('md')} {
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
    </AppBar>
  );
}
