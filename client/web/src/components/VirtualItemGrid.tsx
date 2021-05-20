import { useLayoutEffect, useRef, useState } from "react";
import { css } from "@emotion/react";

export interface Props {
  count: number;
  children: (index: number, styles: React.CSSProperties) => React.ReactNode;
}

export default function VirtualItemGrid({ count, children }: Props) {
  const div = useRef<HTMLDivElement>(null);

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
    const colCount = Math.floor(bounds.width / 136);
    const colWidth = (bounds.width - 8) / colCount;
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
            top: 4 + i * rowHeight,
            left: (j % colCount) * colWidth + 4,
            padding: 4,
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
