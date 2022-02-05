import { style } from "@vanilla-extract/css";

export const overlay = style({
  position: "relative",
  width: "100%",
  height: "100%",
});

export const controls = style({
  position: "absolute",
  bottom: 0,
  left: 0,
  right: 0,
  display: "flex",
  flexDirection: "column",
  padding: 32,
  transition: "all 200ms",
  background: "rgba(0,0,0,0.4)",
});

export const timeBar = style({
  display: "flex",
  alignItems: "center",
});

export const timeText = style({
  lineHeight: "32px",
  fontSize: "0.78em",
  fontFamily: "sans-serif",
  color: "rgb(210,210,210)",
  padding: "0px 8px",
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

export const playPauseButton = style({
  width: "64px",
  height: "64px",
  padding: "0px",
  background: "rgb(20,20,20)",
  borderWidth: "4px",
  borderColor: "orange",
  borderStyle: "inset",
  borderRadius: "50%",
  transition: "all 50ms",
  cursor: "pointer",
  fill: "white",

  ":hover": {
    background: "rgb(30,30,30)",
  },

  ":active": {
    borderWidth: "6px",
  },
});

export const seekButton = style({
  width: "48px",
  height: "48px",
  padding: "0px",
  background: "transparent",
  border: "none",
  borderRadius: 8,
  transition: "all 50ms",
  cursor: "pointer",
  margin: "0px 16px",
  fill: "white",

  ":hover": {
    background: "rgba(255,255,255,0.1)",
  },

  ":active": {
    background: "rgba(255,255,255,0.15)",
  },
});

export const skipButton = style({
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
      fill: "rgb(100,100,100)",
    },

    "&:hover": {
      background: "rgba(255,255,255,0.1)",
    },

    "&:hover:disabled": {
      background: "transparent",
    },

    "&:active": {
      background: "rgba(255,255,255,0.15)",
    },

    "&:active:disabled": {
      background: "transparent",
    },
  },
});
