import { css } from "@emotion/react";
import { useHistory, useLocation } from "react-router";
import {
  Drawer,
  Hidden,
  Icon,
  List,
  ListItem,
  ListItemIcon,
  ListItemText,
  Toolbar,
} from "@material-ui/core";

const WIDTH = 256;

const links = [
  {
    name: "Home",
    icon: "home",
    to: "/",
  },
  {
    name: "Movies",
    icon: "movie",
    to: "/movies",
  },
  {
    name: "Shows",
    icon: "tv",
    to: "/shows",
  },
];

function DrawerContent({ onClose }: { onClose: () => void }) {
  const { push } = useHistory();
  const { pathname } = useLocation();

  function navigate(to: string) {
    if (to !== pathname) {
      push(to);
      onClose();
    }
  }

  return (
    <div
      css={css`
        width: ${WIDTH}px;
      `}
    >
      <List
        css={css`
          padding: 0;
        `}
      >
        {links.map((link) => (
          <ListItem
            key={link.to}
            button
            selected={pathname === link.to}
            onClick={() => navigate(link.to)}
            css={(theme) => css`
              margin: ${theme.spacing(1)};
              width: calc(100% - ${theme.spacing(2)});
              border-radius: 4px;
            `}
          >
            <ListItemIcon>
              <Icon>{link.icon}</Icon>
            </ListItemIcon>
            <ListItemText>{link.name}</ListItemText>
          </ListItem>
        ))}
      </List>
    </div>
  );
}

export interface Props {
  open: boolean;
  onClose: () => void;
}

export default function ({ open, onClose }: Props) {
  return (
    <nav
      css={(theme) => css`
        ${theme.breakpoints.up("md")} {
          width: ${WIDTH}px;
          flex-shrink: 0;
        }
      `}
    >
      <Hidden mdUp>
        <Drawer
          variant="temporary"
          open={open}
          onClose={onClose}
          ModalProps={{ keepMounted: true }}
        >
          <DrawerContent onClose={onClose} />
        </Drawer>
      </Hidden>
      <Hidden mdDown>
        <Drawer variant="permanent" open>
          <Toolbar />
          <DrawerContent onClose={onClose} />
        </Drawer>
      </Hidden>
    </nav>
  );
}
