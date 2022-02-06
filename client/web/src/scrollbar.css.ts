import { style } from "@vanilla-extract/css";

export const scrollbar = style({
  "::-webkit-scrollbar": {
    width: 18,
  },

  "::-webkit-scrollbar-thumb": {
    backgroundColor: "rgba(160, 160, 160, 0.4)",
    backgroundClip: "content-box",
    border: "6px solid transparent",
    borderRadius: 16,
    transition: "all 2000ms",
  },

  selectors: {
    "&::-webkit-scrollbar-thumb:hover": {
      backgroundColor: "rgba(160, 160, 160, 0.5)",
    },
    "&::-webkit-scrollbar-thumb:active": {
      backgroundColor: "rgba(160, 160, 160, 0.8)",
    },
  },
});
