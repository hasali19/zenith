import { Component, JSX, Show, splitProps } from "solid-js";
import { CircleCheckIcon, FileVideoIcon } from "./icons";
import { Image } from "./Image";
import * as styles from "./Poster.css";

export interface PosterProps extends JSX.HTMLAttributes<HTMLDivElement> {
  src: string;
  watched?: boolean;
  clickable?: boolean;
}

export const Poster: Component<PosterProps> = (p) => {
  const [props, rootProps] = splitProps(p, ["src", "watched", "clickable"]);

  const className = () => {
    let className = styles.root;
    if (p.class) {
      className += " " + p.class;
    }
    return className;
  };

  const classList = () => {
    let classList = { [styles.clickable]: props.clickable };
    if (p.classList) {
      classList = { ...classList, ...p.classList };
    }
    return classList;
  };

  return (
    <div class={className()} classList={classList()} {...rootProps}>
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
