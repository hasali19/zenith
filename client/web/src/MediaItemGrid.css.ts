import { style } from "@vanilla-extract/css";
import { scrollbar } from "./scrollbar.css";

export const scrollRoot = style([
  scrollbar,
  {
    width: "100%",
    height: "100%",
    overflow: "auto",
  },
]);

export const outerMargin = style({
  margin: 32,
});

export const item = style({
  padding: 16,
});
