import { Component, JSX, ParentComponent, Show } from "solid-js";
import { Image } from "./Image";
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
    <div class="w-full h-full relative">
      <Image
        src={p.backdrop}
        class="w-full h-full absolute object-cover backdrop-blur"
      />
      <div class="w-full h-full absolute bg-white/70 dark:bg-black/70" />
      <div class="w-full h-full relative overflow-auto p-[5%]">
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
  <div class="flex min-w-0">
    <Poster src={p.poster} watched={p.watched} class="w-[280px]" />
    <div class="flex-1 max-w-[720px] ml-12">
      <h1 class="text-5xl mb-8">{p.name}</h1>
      <h2 class="text-2xl mb-8">{p.subtitle}</h2>
      <Show when={p.actions}>
        <div class="mb-8">{p.actions}</div>
      </Show>
      <p>{p.overview}</p>
    </div>
    <TmdbLink href={p.tmdbLink} class="ml-6" />
  </div>
);

const TmdbLink: Component<{ href: any; class?: string }> = (p) => (
  <div class={p.class}>
    <a href={p.href}>
      <img
        src="https://www.themoviedb.org/assets/2/v4/logos/v2/blue_square_1-5bdc75aaebeb75dc7ae79426ddd9be3b2be1e342510f8202baf6bffa71d7f5c4.svg"
        class="h-8 inline ml-6 mt-3"
      />
    </a>
  </div>
);
