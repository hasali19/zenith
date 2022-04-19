import { style } from "@vanilla-extract/css";

export const overlay = style({
  position: "relative",
  width: "100vw",
  height: "100vh",
});

export const controlsContainer = style({
  position: "absolute",
  bottom: 0,
  left: 0,
  right: 0,
  display: "flex",
  justifyContent: "center",
  padding: 32,
  transition: "all 200ms",
  background: "linear-gradient(transparent, black)",
});

export const controls = style({
  flex: 1,
  maxWidth: 1200,
  display: "flex",
  flexDirection: "column",
});

export const timeBar = style({
  display: "flex",
  alignItems: "center",
});

export const timeText = style({
  lineHeight: "32px",
  fontSize: "0.78em",
  fontFamily: "Exo 2",
  color: "rgb(210,210,210)",
  padding: "0px 8px",
  userSelect: "none",
});

export const seekbarContainer = style({
  flex: 1,
  padding: "0px 8px",
});

export const actionsRow = style({
  display: "grid",
  gridTemplateColumns: "[start] 1fr [mid] auto [end] 1fr",
  justifyContent: "center",
  paddingTop: 16,
  color: "white",
});

export const mainActions = style({
  gridColumn: "mid",
  display: "flex",
  alignItems: "center",
});

export const secondaryActions = style({
  gridColumn: "end",
  display: "flex",
  alignItems: "center",
  justifyContent: "flex-end",
});

export const button = style({
  ":hover": {
    fill: "orange",
  },

  ":active": {
    background: "rgba(255,255,255,0.15)",
  },
});

export const playPauseButton = style([
  button,
  {
    width: 64,
    height: 64,
    padding: 0,
    background: "rgb(20,20,20)",
    border: "none",
    borderRadius: 8,
    transition: "all 50ms",
    cursor: "pointer",
    color: "white",
  },
]);

export const seekButton = style([
  button,
  {
    width: "48px",
    height: "48px",
    padding: "0px",
    background: "transparent",
    border: "none",
    borderRadius: 8,
    transition: "all 50ms",
    cursor: "pointer",
    margin: "0px 16px",
    color: "white",
  },
]);

export const skipButton = style([
  button,
  {
    width: "48px",
    height: "48px",
    padding: "0px",
    background: "transparent",
    border: "none",
    borderRadius: 8,
    transition: "all 50ms",
    cursor: "pointer",

    selectors: {
      "&:disabled": {
        cursor: "default",
        color: "rgb(100,100,100)",
      },

      "&:active:disabled": {
        background: "transparent",
      },
    },
  },
]);
