import { style } from "@vanilla-extract/css";

export const idleUiContainer = style({
  color: "white",
  backgroundColor: "#111111",
  position: "absolute",
  top: "0px",
  bottom: "0px",
  left: "0px",
  right: "0px",
  fontFamily: "sans-serif",
});

export const splashScreen = style({
  width: "100%",
  height: "100%",
  display: "flex",
  flexDirection: "column",
  alignItems: "center",
  justifyContent: "center",
});

export const backdrop = style({
  position: "absolute",
  top: "0px",
  bottom: "0px",
  left: "0px",
  right: "0px",
  backgroundSize: "cover",
  backgroundPosition: "center",
});

export const backdropOverlay = style({
  width: "100%",
  height: "100%",
  backgroundColor: "rgba(0,0,0,0.7)",
});

export const item = style({
  width: "calc(100% - 184px)",
  height: "calc(100% - 184px)",
  position: "absolute",
  margin: "92px",
  display: "flex",
  alignItems: "center",
});

export const poster = style({
  width: "240px",
  objectFit: "cover",
  borderRadius: "8px",
  boxShadow: "2px 2px 8px rgba(40,40,40,0.5)",
});

export const still = style({
  width: "360px",
  objectFit: "cover",
  borderRadius: "8px",
  boxShadow: "2px 2px 8px rgba(40,40,40,0.5)",
});

export const content = style({
  marginLeft: "64px",
  padding: "16px 0px",
  overflow: "scroll",
});

export const pretitle = style({});

export const title = style({
  fontSize: "32pt",
});

export const subtitle = style({
  marginTop: "8px",
  marginBottom: "16px",
  color: "rgb(200,200,200)",
  fontSize: "16pt",
});

export const overview = style({
  fontSize: "16pt",
});
