import { Component, JSX, Show, splitProps } from "solid-js";
import { CircleCheckIcon, FileVideoIcon } from "./icons";
import { Image } from "./Image";

export interface PosterProps extends JSX.HTMLAttributes<HTMLDivElement> {
  src: string;
  watched?: boolean;
}

export const Poster: Component<PosterProps> = (p) => {
  const [props, rootProps] = splitProps(p, ["src", "watched"]);

  const className = () => {
    let className =
      "relative flex items-center justify-center select-none rounded-lg aspect-poster";
    if (p.class) {
      className += " " + p.class;
    }
    return className;
  };

  return (
    <div class={className()} {...rootProps}>
      <Show
        when={props.src}
        fallback={<FileVideoIcon size={56} class="text-white" />}
      >
        {(src) => (
          <Image class="w-100 h-100 object-cover block rounded-lg" src={src} />
        )}
      </Show>
      <Show when={props.watched}>
        <div class="absolute inset-0 bg-black/30 rounded-lg">
          <CircleCheckIcon class="absolute top-0 right-0 m-4 text-white" />
        </div>
      </Show>
    </div>
  );
};
