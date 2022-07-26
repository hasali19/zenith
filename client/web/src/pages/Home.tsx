import { useNavigate } from "solid-app-router";
import { Component, createSignal, For, JSX, onMount } from "solid-js";
import { SwiperOptions } from "swiper";
import { MediaItemWithPoster, MediaItemWithThumbnail } from "../MediaItem";
import preferences from "../preferences";
import { Swiper, SwiperSlide } from "../Swiper";
import { formatYear } from "../utils";

import * as styles from "./Home.css";

const POSTER_SWIPER_BREAKPOINTS = {
  480: { slidesPerView: 3, slidesPerGroup: 3 },
  640: { slidesPerView: 5, slidesPerGroup: 5 },
  1024: { slidesPerView: 6, slidesPerGroup: 6 },
  1400: { slidesPerView: 8, slidesPerGroup: 8 },
};

const THUMBNAIL_SWIPER_BREAKPOINTS = {
  480: { slidesPerView: 1, slidesPerGroup: 1 },
  640: { slidesPerView: 2, slidesPerGroup: 2 },
  1024: { slidesPerView: 3, slidesPerGroup: 3 },
  1400: { slidesPerView: 4, slidesPerGroup: 4 },
};

export const HomeScreen: Component = () => {
  const navigate = useNavigate();

  const [continueWatching, setContinueWatching] = createSignal<any[]>([]);
  const [movies, setMovies] = createSignal<any[]>([]);
  const [shows, setShows] = createSignal<any[]>([]);

  onMount(() => {
    fetch(`${preferences.server}/api/items/continue_watching`)
      .then((res) => res.json())
      .then(setContinueWatching);

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
      <FeaturedSection
        title="Continue Watching"
        breakpoints={THUMBNAIL_SWIPER_BREAKPOINTS}
        items={continueWatching()}
      >
        {(item) => (
          <MediaItemWithThumbnail
            style={{ margin: "4px" }}
            thumbnail={item.thumbnail ?? item.backdrop}
            name={item.title ?? item.show_name}
            secondary={
              item.release_date ? (
                formatYear(item.release_date)
              ) : (
                <span>
                  <span>S{item.season_number.toString().padStart(2, "0")}</span>
                  <span>
                    E{item.episode_number.toString().padStart(2, "0")}
                  </span>
                </span>
              )
            }
            watched={false}
            progress={item.user_data.position / item.video_info.duration}
            onClick={() => {
              let type;
              switch (item.type) {
                case "movie":
                  type = "movies";
                  break;
                case "episode":
                  type = "episodes";
                  break;
                default:
                  console.error(`invalid video item type: ${type}`);
                  return;
              }
              return navigate(`/${type}/${item.id}`);
            }}
          />
        )}
      </FeaturedSection>
      <FeaturedSection
        title="Recent Movies"
        breakpoints={POSTER_SWIPER_BREAKPOINTS}
        items={movies()}
      >
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
      <FeaturedSection
        title="Updated Shows"
        breakpoints={POSTER_SWIPER_BREAKPOINTS}
        items={shows()}
      >
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
  breakpoints: { [width: number]: SwiperOptions };
  children: (item: T) => JSX.Element;
}) {
  return (
    <div class={styles.featuredSection}>
      <h2 class={styles.featuredSectionTitle}>{p.title}</h2>
      <Swiper breakpoints={p.breakpoints}>
        <For each={p.items}>
          {(item) => <SwiperSlide>{p.children(item)}</SwiperSlide>}
        </For>
      </Swiper>
    </div>
  );
}
