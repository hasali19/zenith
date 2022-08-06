import { Component, JSX, ParentComponent, Show } from "solid-js";
import { Image } from "./Image";
import * as styles from "./MediaDetailsScreen.css";
import { Poster } from "./Poster";

export const MediaDetailsScreen: ParentComponent<{
  backdrop: string;
  poster: string;
  name: string;
  subtitle: any;
  overview: string;
  tmdbLink: any;
  watched: boolean;
  headerActions?: JSX.Element;
}> = (p) => {
  return (
    <div class={styles.root}>
      <Image src={p.backdrop} class={styles.backdrop} />
      <div class={styles.backdropOverlay} />
      <div class={styles.content}>
        <HeaderSection
          poster={p.poster}
          name={p.name}
          subtitle={p.subtitle}
          overview={p.overview}
          tmdbLink={p.tmdbLink}
          watched={p.watched}
          actions={p.headerActions}
        />
        {p.children}
      </div>
    </div>
  );
};

const HeaderSection: Component<{
  poster: string;
  name: string;
  subtitle: any;
  overview: string;
  tmdbLink: any;
  watched: boolean;
  actions?: JSX.Element;
}> = (p) => (
  <div class={styles.headerSection}>
    <Poster src={p.poster} watched={p.watched} class={styles.poster} />
    <div class={styles.headerContentContainer}>
      <h1>{p.name}</h1>
      <h2>{p.subtitle}</h2>
      <Show when={p.actions}>
        <div style={{ "margin-top": "32px" }}>{p.actions}</div>
      </Show>
      <p class={styles.headerSectionOverview}>{p.overview}</p>
    </div>
    <TmdbLink href={p.tmdbLink} class={styles.tmdbLinkBox} />
  </div>
);

const TmdbLink: Component<{ href: any; class?: string }> = (p) => (
  <div class={p.class}>
    <a href={p.href}>
      <img
        src="https://www.themoviedb.org/assets/2/v4/logos/v2/blue_square_1-5bdc75aaebeb75dc7ae79426ddd9be3b2be1e342510f8202baf6bffa71d7f5c4.svg"
        class={styles.tmdbLinkImg}
      />
    </a>
  </div>
);
