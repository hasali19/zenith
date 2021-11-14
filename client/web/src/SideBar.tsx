import { useLocation, useNavigate } from "solid-app-router";
import { Component, createMemo, For } from "solid-js";
import * as styles from "./SideBar.css";

const MENU_ITEMS = [
  { name: "Home", icon: "home", path: "/" },
  { name: "Movies", icon: "movie", path: "/movies" },
  { name: "Shows", icon: "tv", path: "/shows" },
];

export const SideBar: Component = () => {
  const location = useLocation();
  const navigate = useNavigate();

  const current = createMemo(() => {
    let current = 0;
    for (let i = 0; i < MENU_ITEMS.length; i++) {
      if (location.pathname.startsWith(MENU_ITEMS[i].path)) {
        current = i;
      }
    }
    return current;
  });

  return (
    <div className={styles.sideBar}>
      <For each={MENU_ITEMS}>
        {({ icon, name, path }) => (
          <div
            className={styles.sideBarItem}
            onClick={() => navigate(path, { replace: true })}
          >
            <span className="material-icons">{icon}</span>
            {name}
          </div>
        )}
      </For>
      <div
        className={styles.sideBarItemIndicator}
        style={{ transform: `translateY(${current() * 72}px)` }}
      />
    </div>
  );
};
