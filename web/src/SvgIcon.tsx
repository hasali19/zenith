import { splitProps } from "solid-js";
import { JSX } from "solid-js/jsx-runtime";
import { Component } from "solid-js/types";

export interface SvgIconProps extends JSX.SvgSVGAttributes<SVGSVGElement> {
  path: string;
  size?: number;
  viewBox?: string;
}

export const SvgIcon: Component<SvgIconProps> = (p) => {
  const [props, childProps] = splitProps(p, ["size", "class"]);
  const size = () => props.size || 24;
  const viewBox = p.viewBox || "0 0 24 24";
  return (
    <svg
      class={`fill-current ${props.class ?? ""}`}
      width={size()}
      height={size()}
      viewBox={viewBox}
      {...childProps}
    >
      <path d={p.path} />
    </svg>
  );
};
