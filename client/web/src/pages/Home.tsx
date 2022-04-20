import { useNavigate } from "solid-app-router";
import { Component, createSignal, For, JSX, onMount } from "solid-js";
import { MediaItemWithPoster } from "../MediaItem";
import preferences from "../preferences";
import { Swiper, SwiperSlide } from "../Swiper";
import { formatYear } from "../utils";

import * as styles from "./Home.css";

export const HomeScreen: Component = () => {
  const navigate = useNavigate();

  const [movies, setMovies] = createSignal<any[]>([]);
  const [shows, setShows] = createSignal<any[]>([]);

  onMount(() => {
    fetch(`${preferences.server}/api/movies/recent`)
      .then((res) => res.json())
      .then(setMovies);

    fetch(`${preferences.server}/api/tv/shows/recent`)
      .then((res) => res.json())
      .then(setShows);
  });

  return (
    <div style={{ padding: "32px" }}>
      <HeroSection item={movies()[0]} />
      <FeaturedSection title="Recent Movies" items={movies()}>
        {(item) => (
          <MediaItemWithPoster
            style={{ margin: "4px" }}
            poster={item.poster}
            name={item.title}
            secondary={formatYear(item.release_date)}
            watched={item.user_data.is_watched}
            onClick={() => navigate(`/movies/${item.id}`)}
          />
        )}
      </FeaturedSection>
      <FeaturedSection title="Updated Shows" items={shows()}>
        {(item) => (
          <MediaItemWithPoster
            style={{ margin: "4px" }}
            poster={item.poster}
            name={item.name}
            secondary={formatYear(item.start_date)}
            watched={item.user_data.unwatched === 0}
            onClick={() => navigate(`/shows/${item.id}`)}
          />
        )}
      </FeaturedSection>
    </div>
  );
};

const HeroSection: Component<{ item?: any }> = (p) => (
  <>
    {p.item && (
      <div
        class={styles.heroSection}
        style={{ "background-image": `url(${p.item.backdrop})` }}
      />
    )}
  </>
);

function FeaturedSection<T>(p: {
  title: string;
  items: T[];
  children: (item: T) => JSX.Element;
}) {
  return (
    <div class={styles.featuredSection}>
      <h2 class={styles.featuredSectionTitle}>{p.title}</h2>
      <Swiper>
        <For each={p.items}>
          {(item) => <SwiperSlide>{p.children(item)}</SwiperSlide>}
        </For>
      </Swiper>
    </div>
  );
}
