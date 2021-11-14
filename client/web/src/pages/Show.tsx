import { useNavigate, useParams } from "solid-app-router";
import { Component, createEffect, createSignal, For, Show } from "solid-js";
import { MediaDetailsScreen } from "../MediaDetailsScreen";
import { MediaItemWithPoster } from "../MediaItem";
import preferences from "../preferences";
import { formatYear } from "../utils";
import * as styles from "./Show.css";

async function fetchShow(id: string) {
  const url = `${preferences.server}/api/tv/shows/${id}`;
  const res = await fetch(url);
  return await res.json();
}

async function fetchSeasons(showId: string) {
  const url = `${preferences.server}/api/tv/shows/${showId}/seasons`;
  const res = await fetch(url);
  const seasons = await res.json();

  for (const season of seasons) {
    season.episode_count = await fetchEpisodeCount(season.id);
  }

  return await seasons;
}

async function fetchEpisodeCount(seasonId: string) {
  const url = `${preferences.server}/api/tv/seasons/${seasonId}/episodes`;
  const res = await fetch(url);
  const episodes = await res.json();
  return episodes.length;
}

export const ShowScreen: Component = () => {
  const params = useParams();
  const navigate = useNavigate();

  const [show, setShow] = createSignal<any | null>(null);
  const [seasons, setSeasons] = createSignal<any[]>([]);

  createEffect(() => {
    fetchShow(params.id).then(setShow);
    fetchSeasons(params.id).then(setSeasons);
  });

  return (
    <Show when={show()}>
      {(show) => (
        <MediaDetailsScreen
          name={show.name}
          poster={show.poster}
          backdrop={show.backdrop}
          subtitle={formatYear(show.start_date)}
          overview={show.overview}
          tmdbLink={`https://www.themoviedb.org/tv/${show.external_ids.tmdb}`}
        >
          <SeasonsSection
            seasons={seasons()}
            onItemClick={(season) => navigate(`/seasons/${season.id}`)}
          />
        </MediaDetailsScreen>
      )}
    </Show>
  );
};

const SeasonsSection: Component<{
  seasons: any[];
  onItemClick: (item: any) => void;
}> = (p) => (
  <div class={styles.seriesSection}>
    <h3 class={styles.seriesHeading}>Series</h3>
    <div class={styles.seriesGrid}>
      <For each={p.seasons}>
        {(season) => (
          <div class={styles.seriesItemWrapper}>
            <MediaItemWithPoster
              poster={season.poster}
              name={season.name}
              secondary={`${season.episode_count} episodes`}
              onClick={() => p.onItemClick(season)}
            />
          </div>
        )}
      </For>
    </div>
  </div>
);
