import { style } from "@vanilla-extract/css";

export const sideBar = style({
  width: 80,
  position: "relative",
  display: "flex",
  flexDirection: "column",
  padding: 4,
  boxSizing: "border-box",
  backgroundColor: "rgba(255, 255, 255, 0.03)",
});

export const sideBarItemIndicator = style({
  width: 64,
  height: 64,
  position: "absolute",
  borderRadius: 8,
  margin: 4,
  backgroundColor: "rgba(255, 255, 255, 0.1)",
  zIndex: 0,
  transition: "transform 200ms ease-in",
});

export const sideBarItem = style({
  width: 64,
  height: 64,
  borderRadius: 8,
  margin: 4,
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
  justifyContent: "center",
  userSelect: "none",
  fontSize: "0.8rem",
  zIndex: 1,
  transition: "all 50ms",
  ":hover": {
    backgroundColor: "rgba(255, 255, 255, 0.05)",
    transform: "scale(0.95)",
  },
  ":active": {
    transform: "scale(0.9)",
  },
});
