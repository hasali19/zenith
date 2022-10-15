import { Component, JSX } from "solid-js";

export const Image: Component<JSX.ImgHTMLAttributes<HTMLImageElement>> = (
  p
) => {
  return (
    <img
      {...p}
      class={`${p.class ?? ""} transition-opacity duration-300 opacity-0`}
      onLoad={(e) => e.currentTarget.classList.add("opacity-100")}
    />
  );
};
