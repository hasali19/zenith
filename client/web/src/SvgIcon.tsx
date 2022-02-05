import { JSX } from "solid-js/jsx-runtime";
import { Component } from "solid-js/types";

export const SvgIcon: Component<
  { path: string; size?: number } & JSX.SvgSVGAttributes<SVGSVGElement>
> = (p) => {
  const size = () => p.size || 24;
  return (
    <svg width={size()} height={size()} viewBox={`0 0 24 24`} {...p}>
      <path d={p.path} />
    </svg>
  );
};
