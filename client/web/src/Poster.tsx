import { Component, JSX, Show, splitProps } from "solid-js";
import { CircleCheckIcon, FileVideoIcon } from "./icons";
import { Image } from "./Image";
import * as styles from "./Poster.css";

export interface PosterProps extends JSX.HTMLAttributes<HTMLDivElement> {
  src: string;
  watched?: boolean;
}

export const Poster: Component<PosterProps> = (p) => {
  const [props, rootProps] = splitProps(p, ["src", "watched"]);

  const className = () => {
    let className = styles.root;
    if (p.class) {
      className += " " + p.class;
    }
    return className;
  };

  return (
    <div class={className()} {...rootProps}>
      <Show
        when={props.src}
        fallback={<FileVideoIcon size={56} style={{ color: "white" }} />}
      >
        {(src) => <Image class={styles.image} src={src} />}
      </Show>
      <Show when={props.watched}>
        <div class={styles.overlay}>
          <CircleCheckIcon class={styles.posterCheck} />
        </div>
      </Show>
    </div>
  );
};
