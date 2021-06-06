import { useLayoutEffect, useRef, useState } from "react";
import { css, Theme } from "@emotion/react";
import { useMediaQuery } from "@material-ui/core";

export interface Props {
  count: number;
  children: (index: number, styles: React.CSSProperties) => React.ReactNode;
}

export default function VirtualItemGrid({ count, children }: Props) {
  const div = useRef<HTMLDivElement>(null);
  const desktop = useMediaQuery((theme: Theme) => theme.breakpoints.up("md"));

  const [bounds, setBounds] = useState<DOMRect | null>(null);
  const [scroll, setScroll] = useState(0);

  useLayoutEffect(() => {
    const root = div.current;

    if (!root) {
      return;
    }

    const observer = new ResizeObserver(() => {
      setBounds(root.getBoundingClientRect());
    });

    observer.observe(root);

    return () => {
      observer.unobserve(root);
    };
  }, []);

  let content = null;

  if (bounds && bounds.width > 0 && bounds.height > 0 && count > 0) {
    let padding = 4;
    let targetColWidth = 136;

    if (desktop) {
      targetColWidth = 172;
    }

    const colCount = Math.floor(bounds.width / targetColWidth);
    const colWidth = (bounds.width - padding * 2) / colCount;
    const rowCount = Math.ceil(count / colCount);
    const rowHeight = colWidth * 1.5 + 64;

    const items = [];

    const start = Math.max(0, Math.floor(scroll / rowHeight) - 2);
    const end = Math.min(
      rowCount - 1,
      Math.floor((scroll + bounds.height) / rowHeight) + 2
    );

    for (let i = start; i <= end; i++) {
      const startIndex = i * colCount;
      const endIndex = Math.min((i + 1) * colCount, count);

      for (let j = startIndex; j < endIndex; j++) {
        items.push(
          children(j, {
            width: colWidth,
            height: rowHeight,
            position: "absolute",
            top: padding + i * rowHeight,
            left: (j % colCount) * colWidth + padding,
            padding,
          })
        );
      }
    }

    content = (
      <div style={{ height: rowCount * rowHeight, position: "relative" }}>
        {items}
      </div>
    );
  }

  return (
    <div
      onScroll={(e) => setScroll(e.currentTarget.scrollTop)}
      css={css`
        height: 100%;
        overflow: auto;
      `}
    >
      <div
        ref={div}
        css={css`
          width: 100%;
          height: 100%;
        `}
      >
        {content}
      </div>
    </div>
  );
}
