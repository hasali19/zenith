import { useNavigate } from "solid-app-router";
import {
  Component,
  createEffect,
  createResource,
  createSignal,
  For,
  JSX,
  onCleanup,
  Suspense,
  untrack,
} from "solid-js";
import { SwiperOptions } from "swiper";
import { MediaItemWithPoster, MediaItemWithThumbnail } from "../MediaItem";
import preferences from "../preferences";
import { Swiper, SwiperSlide } from "../Swiper";
import { formatYear } from "../utils";

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

  const [data] = createResource(() =>
    Promise.all([
      fetch(`${preferences.server}/api/items/continue_watching`).then((res) =>
        res.json()
      ),
      fetch(`${preferences.server}/api/movies/recent`).then((res) =>
        res.json()
      ),
      fetch(`${preferences.server}/api/tv/shows/recent`).then((res) =>
        res.json()
      ),
    ])
  );

  const continueWatching = () => (data()?.[0] as any[]) ?? [];
  const movies = () => (data()?.[1] as any[]) ?? [];
  const shows = () => (data()?.[2] as any[]) ?? [];

  const [hero, setHero] = createSignal(0);

  createEffect(() => {
    const n = movies().length;
    const interval = setInterval(() => setHero((i) => (i + 1) % n), 10_000);
    onCleanup(() => clearInterval(interval));
  });

  return (
    <Suspense>
      <div style={{ padding: "32px" }}>
        <HeroSection item={movies()[hero()]} />
        <FeaturedSection
          title="Continue Watching"
          breakpoints={THUMBNAIL_SWIPER_BREAKPOINTS}
          items={continueWatching()}
        >
          {(item) => (
            <MediaItemWithThumbnail
              style={{ margin: "4px" }}
              thumbnail={`${preferences.server}/api/items/${item.id}/images/thumbnail`}
              name={item.title ?? item.show_name}
              secondary={
                item.release_date ? (
                  formatYear(item.release_date)
                ) : (
                  <span>
                    <span>
                      S{item.season_number.toString().padStart(2, "0")}
                    </span>
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
              poster={`${preferences.server}/api/items/${item.id}/images/poster`}
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
              poster={`${preferences.server}/api/items/${item.id}/images/poster`}
              name={item.name}
              secondary={formatYear(item.start_date)}
              watched={item.user_data.unwatched === 0}
              onClick={() => navigate(`/shows/${item.id}`)}
            />
          )}
        </FeaturedSection>
      </div>
    </Suspense>
  );
};

const HeroSection: Component<{ item?: any }> = (p) => {
  const [stack, setStack] = createSignal<any[]>([]);

  createEffect(() => {
    const prev = untrack(stack)[0];
    if (p.item) {
      setStack((s) => [...s, p.item]);
    }
    if (prev) {
      setTimeout(() => setStack((s) => s.filter((i) => i !== prev)), 1000);
    }
  });

  return (
    <>
      {
        <div class="w-100 h-[40vh] relative shadow-xl rounded-lg m-2">
          <For each={stack()}>
            {(item) => (
              <img
                src={`${preferences.server}/api/items/${item.id}/images/backdrop`}
                class="w-full h-full absolute rounded-lg object-cover transition-opacity duration-1000 opacity-0"
                onLoad={(e) => e.currentTarget.classList.add("opacity-100")}
              />
            )}
          </For>
        </div>
      }
    </>
  );
};

function FeaturedSection<T>(p: {
  title: string;
  items: T[];
  breakpoints: { [width: number]: SwiperOptions };
  children: (item: T) => JSX.Element;
}) {
  return (
    <div class="p-1">
      <h2 class="text-2xl m-1 mt-6 mb-3">{p.title}</h2>
      <Swiper breakpoints={p.breakpoints}>
        <For each={p.items}>
          {(item) => <SwiperSlide>{p.children(item)}</SwiperSlide>}
        </For>
      </Swiper>
    </div>
  );
}
