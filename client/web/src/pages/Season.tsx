import { useNavigate, useParams } from "solid-app-router";
import { Component, createEffect, createSignal, For, Show } from "solid-js";
import { MediaDetailsScreen } from "../MediaDetailsScreen";
import { MediaItemWithThumbnail } from "../MediaItem";
import preferences from "../preferences";
import * as styles from "./Season.css";

async function fetchShow(id: string) {
  const url = `${preferences.server}/api/tv/shows/${id}`;
  const res = await fetch(url);
  return await res.json();
}

async function fetchSeason(id: string) {
  const url = `${preferences.server}/api/tv/seasons/${id}`;
  const res = await fetch(url);
  return await res.json();
}

async function fetchEpisodes(seasonId: string) {
  const url = `${preferences.server}/api/tv/seasons/${seasonId}/episodes`;
  const res = await fetch(url);
  return await res.json();
}

function formatSeasonTitle(season: any) {
  if (season.name) {
    if (/(?:Season|Series) +\d+/.test(season.name)) {
      return season.name;
    } else {
      return `Season ${season.season_number} - ${season.name}`;
    }
  } else {
    return season.name;
  }
}

export const SeasonScreen: Component = () => {
  const params = useParams();
  const navigate = useNavigate();

  const [show, setShow] = createSignal<any | null>(null);
  const [season, setSeason] = createSignal<any | null>(null);
  const [episodes, setEpisodes] = createSignal<any[]>([]);

  createEffect(() => {
    fetchSeason(params.id).then((season) => {
      setSeason(season);
      fetchShow(season.show_id).then(setShow);
    });
    fetchEpisodes(params.id).then(setEpisodes);
  });

  const data = () =>
    show() && season() ? { show: show(), season: season() } : null;

  return (
    <Show when={data()}>
      {({ show, season }) => (
        <MediaDetailsScreen
          name={show.name}
          poster={season.poster || show.poster}
          backdrop={season.backdrop || show.backdrop}
          subtitle={formatSeasonTitle(season)}
          overview={season.overview || show.overview}
          watched={season.user_data.unwatched === 0}
          tmdbLink={`https://www.themoviedb.org/tv/${show.external_ids.tmdb}/season/${season.season_number}`}
        >
          <EpisodesSection
            episodes={episodes()}
            onItemClick={(item) => navigate(`/episodes/${item.id}`)}
          />
        </MediaDetailsScreen>
      )}
    </Show>
  );
};

const EpisodesSection: Component<{
  episodes: any[];
  onItemClick: (item: any) => void;
}> = (p) => (
  <div class={styles.seriesSection}>
    <h3>Episodes</h3>
    <div class={styles.seriesGrid}>
      <For each={p.episodes}>
        {(episode) => (
          <MediaItemWithThumbnail
            thumbnail={episode.thumbnail || episode.backdrop}
            name={episode.name}
            secondary={`Episode ${episode.episode_number}`}
            watched={episode.user_data.is_watched}
            onClick={[p.onItemClick, episode]}
          />
        )}
      </For>
    </div>
  </div>
);
