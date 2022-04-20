import { VirtualContainer } from "@minht11/solid-virtual-container";
import { Component } from "solid-js";
import * as styles from "./MediaItemGrid.css";
import { MediaItemWithPoster } from "./MediaItem";
import { formatYear } from "./utils";

export interface MediaItem {
  id: number;
  name: string;
  date: any;
  poster: string;
  watched: boolean;
}

export const MediaItemGrid: Component<{
  items: MediaItem[];
  onItemClick: (item: MediaItem) => void;
}> = (p) => {
  let root!: HTMLDivElement;

  function calculateItemSize(crossAxisSize: number) {
    const minWidth = (crossAxisSize > 560 ? 180 : 140) + 32;

    const count = Math.floor(crossAxisSize / minWidth);
    const width = Math.floor(crossAxisSize / count);

    return {
      width,
      height: (width * 3) / 2 + 72,
    };
  }

  return (
    <div ref={root} class={styles.scrollRoot}>
      <div class={styles.outerMargin}>
        <VirtualContainer
          items={p.items}
          scrollTarget={root}
          itemSize={calculateItemSize}
          crossAxisCount={(m) =>
            Math.floor(m.container.cross / m.itemSize.cross)
          }
        >
          {(q) => (
            <div class={styles.item} style={q.style}>
              <MediaItemWithPoster
                poster={q.item.poster}
                name={q.item.name}
                secondary={formatYear(q.item.date)}
                watched={q.item.watched}
                onClick={() => p.onItemClick(q.item)}
              />
            </div>
          )}
        </VirtualContainer>
      </div>
    </div>
  );
};
