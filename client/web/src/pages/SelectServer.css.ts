import { style } from "@vanilla-extract/css";

export const root = style({
  flex: 1,
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
});

export const form = style({
  flex: 1,
  maxWidth: 480,
  display: "flex",
  flexDirection: "column",
});

export const title = style({
  marginTop: 16,
  marginBottom: 16,
  textAlign: "center",
  fontWeight: "normal",
});

export const input = style({
  backgroundColor: "rgba(255, 255, 255, 0.1)",
  borderRadius: 4,
  color: "white",
  ":focus-visible": {
    backgroundColor: "rgba(50, 50, 50, 0.6)",
  },
  "::placeholder": {
    color: "grey",
  },
});

export const button = style({
  marginTop: 16,
  selectors: {
    "&.button:disabled": {
      backgroundColor: "#d16318",
    },
  },
});
