import { useNavigate, useParams } from "solid-app-router";
import { Component, createEffect, createSignal, Show } from "solid-js";
import { MediaDetailsScreen } from "../MediaDetailsScreen";
import { PlayButton } from "../PlayButton";
import preferences from "../preferences";
import { formatDuration } from "../utils";

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

async function fetchEpisode(id: string) {
  const url = `${preferences.server}/api/tv/episodes/${id}`;
  const res = await fetch(url);
  return await res.json();
}

export const EpisodeScreen: Component = () => {
  const params = useParams();
  const navigate = useNavigate();

  const [show, setShow] = createSignal<any | null>(null);
  const [season, setSeason] = createSignal<any | null>(null);
  const [episode, setEpisode] = createSignal<any | null>(null);

  createEffect(() => {
    fetchEpisode(params.id).then((episode) => {
      setEpisode(episode);
      fetchShow(episode.show_id).then(setShow);
      fetchSeason(episode.season_id).then(setSeason);
    });
  });

  const data = () =>
    show() && season() && episode()
      ? { show: show(), season: season(), episode: episode() }
      : null;

  return (
    <Show when={data()}>
      {({ show, season, episode }) => (
        <MediaDetailsScreen
          name={episode.name}
          poster={season.poster || show.poster}
          backdrop={season.backdrop || show.backdrop}
          subtitle={
            <span>
              <span>S{episode.season_number.toString().padStart(2, "0")}</span>
              <span>E{episode.episode_number.toString().padStart(2, "0")}</span>
              <span style={{ margin: "0px 16px" }}>·</span>
              <span>{formatDuration(episode.video_info.duration)}</span>
            </span>
          }
          overview={episode.overview}
          tmdbLink={`https://www.themoviedb.org/tv/${show.external_ids.tmdb}/season/${episode.season_number}/episode/${episode.episode_number}`}
          headerActions={
            <>
              <PlayButton onClick={() => navigate(`/player/${params.id}`)} />
            </>
          }
        />
      )}
    </Show>
  );
};
