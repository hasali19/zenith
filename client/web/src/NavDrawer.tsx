import { useLocation, useNavigate } from "solid-app-router";
import { Component, JSX, ParentComponent } from "solid-js";
import { FilmIcon, GearIcon, HomeIcon, TvIcon } from "./icons";
import * as styles from "./NavDrawer.css";

export const NavDrawer: Component = () => {
  const location = useLocation();
  const navigate = useNavigate();

  function isPath(path: string) {
    return location.pathname === path;
  }

  function onNavigate(path: string) {
    if (!isPath(path)) {
      navigate(path);
    }
  }

  const SubTitle: ParentComponent = (p) => {
    return <p class={styles.subtitle}>{p.children}</p>;
  };

  const Section: ParentComponent = (p) => {
    return <div class={styles.section}>{p.children}</div>;
  };

  interface LinkProps {
    title: string;
    path: string;
    icon: JSX.Element;
  }

  const Link: Component<LinkProps> = (p) => {
    return (
      <div
        class={styles.link}
        classList={{ [styles.active]: isPath(p.path) }}
        onClick={[onNavigate, p.path]}
      >
        <div class={styles.icon}>{p.icon}</div>
        {p.title}
      </div>
    );
  };

  const Divider: Component = () => {
    return <div class={styles.divider} />;
  };

  return (
    <div class={styles.drawer}>
      <div class={styles.header}>
        <img src="/images/zenith.png" class={styles.img} />
      </div>
      <Divider />
      <Section>
        <SubTitle>General</SubTitle>
        <Link title="Home" path="/" icon={<HomeIcon size={20} />} />
      </Section>
      <Section>
        <SubTitle>Libraries</SubTitle>
        <Link title="Movies" path="/movies" icon={<FilmIcon size={20} />} />
        <Link title="Shows" path="/shows" icon={<TvIcon size={20} />} />
      </Section>
      <Section>
        <SubTitle>System</SubTitle>
        <Link title="Settings" path="/settings" icon={<GearIcon size={20} />} />
      </Section>
    </div>
  );
};
