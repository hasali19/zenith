import { useState } from "react";
import { Link } from "react-router-dom";
import styled from "styled-components";

import logo from "../images/logo.png";

function isMobile() {
  return window.innerWidth < 600;
}

function MenuButton({ onClick }: { onClick: () => void }) {
  return (
    <AppLogo alt="App Logo" src={logo} className="app-logo" onClick={onClick} />
  );
}

interface DrawerLinkProps {
  to: string;
  children: React.ReactNode;
}

export interface Props {
  children: React.ReactNode;
}

export function AppContainer({ children }: Props) {
  const [open, setOpen] = useState(false);

  function DrawerLink({ to, children }: DrawerLinkProps) {
    return (
      <UnstyledLink to={to} onClick={() => setOpen(false)}>
        <NavDrawerItem>{children}</NavDrawerItem>
      </UnstyledLink>
    );
  }

  return (
    <AppContainerDiv>
      <AppBar>
        <MenuButton onClick={() => isMobile() && setOpen((v) => !v)} />
        <AppTitle>Zenith</AppTitle>
      </AppBar>

      {open && (
        <NavDrawerMainOverlay onClick={() => isMobile() && setOpen(false)} />
      )}

      <NavContainer open={open}>
        <NavDrawer>
          <AppBarSpacer></AppBarSpacer>
          <NavDrawerContent>
            <DrawerLink to="/">Home</DrawerLink>
            <DrawerLink to="/movies">Movies</DrawerLink>
            <DrawerLink to="/tv_shows">TV Shows</DrawerLink>
          </NavDrawerContent>
        </NavDrawer>
      </NavContainer>

      <Main>
        <AppBarSpacer></AppBarSpacer>
        <div>{children}</div>
      </Main>
    </AppContainerDiv>
  );
}

const AppContainerDiv = styled.div`
  min-height: 100vh;
  background: #2a2a2a;
  color: #eeeeee;
  display: flex;
`;

const APP_BAR_HEIGHT = 50;

const AppBarSpacer = styled.div`
  width: 100%;
  height: ${APP_BAR_HEIGHT}px;
`;

const AppBar = styled.div`
  width: 100%;
  height: ${APP_BAR_HEIGHT}px;
  position: fixed;
  background-color: #151515;
  z-index: 200;
  display: flex;
  align-items: center;
  box-shadow: 0px 4px 4px #111111;
`;

const AppLogo = styled.img`
  margin: 8px 16px;
  height: calc(100% - 16px);
`;

const AppTitle = styled.h2`
  font-weight: 500;
`;

const NavContainer = styled.div<{ open: boolean }>`
  width: 240px;
  transition: left 300ms;

  @media (max-width: 599px) {
    position: fixed;
    left: ${(props) => (props.open ? 0 : -240)}px;
  }
`;

const NavDrawer = styled.div`
  width: 240px;
  height: 100vh;
  background: #1f1f1f;
  position: fixed;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  z-index: 100;
`;

const NavDrawerContent = styled.div`
  overflow: auto;
  flex: 1;
`;

const NavDrawerItem = styled.div`
  user-select: none;
  padding: 16px;
  display: flex;
  align-items: center;
  cursor: pointer;

  &:hover {
    background: #333333;
  }

  &:active {
    background: #533c3c;
  }
`;

const NavDrawerMainOverlay = styled.div`
  position: fixed;
  width: 100vw;
  height: 100vh;
  background: #00000055;
`;

const Main = styled.main`
  flex: 1;
`;

const UnstyledLink = styled(Link)`
  text-decoration: none;
  color: inherit;
`;
