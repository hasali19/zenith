import { style } from "@vanilla-extract/css";

export const button = style({
  padding: "12px 24px",
  display: "flex",
  justifyContent: "center",
  alignItems: "center",
  background: "#ff7447",
  lineHeight: "1em",
  fontFamily: "inherit",
  fontWeight: "bold",
  color: "white",
  border: "none",
  borderRadius: 20,
  cursor: "pointer",
  transition: "background 200ms, transform 200ms",

  ":hover": {
    background: "#ff8c67",
  },

  ":active": {
    transform: "scale(0.9)",
  },
});

export const icon = style({
  marginRight: 16,
});
