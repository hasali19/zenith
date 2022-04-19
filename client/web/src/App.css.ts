import { style } from "@vanilla-extract/css";

export const app = style({
  width: "100vw",
  height: "100vh",
  display: "grid",
  gridTemplateColumns: "256px minmax(0, 1fr)",
});

export const content = style({
  height: "100vh",
  overflow: "auto",
});
