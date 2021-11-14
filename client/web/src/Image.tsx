import { Component, JSX } from "solid-js";
import * as styles from "./Image.css";

export const Image: Component<JSX.ImgHTMLAttributes<HTMLImageElement>> = (
  p
) => {
  return (
    <img
      {...p}
      class={`${p.class ?? ""} ${styles.img}`}
      onLoad={(e) => e.currentTarget.classList.add(styles.loaded)}
    />
  );
};
