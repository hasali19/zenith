import { style } from "@vanilla-extract/css";

export const drawer = style({
  display: "flex",
  flexDirection: "column",
  padding: 8,
  backgroundColor: "#222222",
  boxShadow: "0px 0px 4px #141414",
});

export const active = style({
  background: "rgba(255,255,255,0.2)",
  color: "orange",
});

export const header = style({
  padding: 32,
  display: "flex",
  justifyContent: "center",
});

export const img = style({
  width: 64,
  height: 88,
});

export const divider = style({
  borderTop: "1px solid #484848",
  margin: "8px 0px",
});

export const section = style({
  padding: 4,
});

export const subtitle = style({
  fontSize: "0.85em",
  fontWeight: "bold",
  color: "lightgray",
});

export const link = style({
  margin: "4px 0px",
  padding: "12px",
  display: "flex",
  alignItems: "center",
  userSelect: "none",
  fontSize: "0.95rem",
  transition: "background 200ms",
  borderRadius: 8,
  cursor: "pointer",
  selectors: {
    [`&:not(${active}):hover`]: {
      background: "rgba(255,255,255,0.1)",
    },
  },
});

export const icon = style({
  marginRight: 16,
  display: "flex",
});
