import { style } from "@vanilla-extract/css";

export const app = style({
  flex: 1,
  display: "flex",
  minWidth: 0,
  backgroundColor: "rgba(0, 0, 0, 0.3)",
});

export const mainContent = style({
  flex: 1,
  display: "flex",
  flexDirection: "column",
  minWidth: 0,
  height: "100vh",
  overflow: "auto",
  boxSizing: "border-box",
});
