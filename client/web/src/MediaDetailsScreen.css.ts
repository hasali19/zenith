import { style } from "@vanilla-extract/css";

export const root = style({
  width: "100%",
  height: "100%",
  position: "relative",
});

export const backdrop = style({
  width: "100%",
  height: "100%",
  position: "absolute",
  objectFit: "cover",
  filter: "blur(8px)",
});

export const backdropOverlay = style({
  width: "100%",
  height: "100%",
  position: "absolute",
  backgroundColor: "rgba(0, 0, 0, 0.7)",

  "@media": {
    "(prefers-color-scheme: light)": {
      backgroundColor: "rgba(255,255,255,0.7)",
    },
  },
});

export const content = style({
  width: "100%",
  height: "100%",
  position: "relative",
  overflow: "auto",
  padding: "5%",
  boxSizing: "border-box",
});

export const headerSection = style({
  display: "flex",
  minWidth: 0,
});

export const poster = style({
  width: 280,
});

export const headerContentContainer = style({
  flex: 1,
  maxWidth: 720,
  marginLeft: 64,
});

export const headerSectionTitle = style(["title", "is-1"]);

export const headerSectionSubtitle = style(["subtitle", "is-3"]);

export const headerSectionOverview = style({
  marginTop: 32,
});

export const tmdbLinkBox = style({
  marginLeft: 32,
});

export const tmdbLinkImg = style({
  height: 32,
  display: "inline",
  marginLeft: 24,
  marginTop: 12,
});
