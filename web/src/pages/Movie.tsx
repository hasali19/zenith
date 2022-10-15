import { useNavigate, useParams } from "solid-app-router";
import { Component, createEffect, createSignal, Show } from "solid-js";
import { MediaDetailsScreen } from "../MediaDetailsScreen";
import { PlayButton } from "../PlayButton";
import preferences from "../preferences";
import { formatDuration, formatPosition, formatYear } from "../utils";

export const MovieScreen: Component = () => {
  const params = useParams();
  const navigate = useNavigate();
  const [movie, setMovie] = createSignal<any | null>(null);

  createEffect(() => {
    fetch(`${preferences.server}/api/movies/${params.id}`)
      .then((res) => res.json())
      .then(setMovie);
  });

  return (
    <Show when={movie()}>
      {(movie) => (
        <MediaDetailsScreen
          name={movie.title}
          poster={`${preferences.server}/api/items/${movie.id}/images/poster`}
          backdrop={`${preferences.server}/api/items/${movie.id}/images/backdrop`}
          subtitle={
            <span>
              <span>{formatYear(movie.release_date)}</span>
              <span style={{ margin: "0px 16px" }}>·</span>
              <span>{formatDuration(movie.video_info.duration)}</span>
            </span>
          }
          overview={movie.overview}
          tmdbLink={`https://www.themoviedb.org/movie/${movie.external_ids.tmdb}`}
          watched={movie.user_data.is_watched}
          headerActions={
            <div style={{ display: "inline-flex", "flex-direction": "column" }}>
              <div>
                <PlayButton
                  resume={shouldResume(movie)}
                  onClick={() => {
                    let query = shouldResume(movie)
                      ? `?start=${movie.user_data.position}`
                      : "";
                    return navigate(`/player/${params.id}${query}`);
                  }}
                />
              </div>
              <Show when={shouldResume(movie)}>
                <span
                  style={{
                    "font-size": "0.8em",
                    "text-align": "center",
                    "margin-top": "4px",
                  }}
                >
                  {formatPosition(
                    movie.user_data.position,
                    movie.video_info.duration
                  )}
                </span>
              </Show>
            </div>
          }
        />
      )}
    </Show>
  );
};

function shouldResume(movie: any) {
  return (
    movie.user_data.position > 0.1 * movie.video_info.duration &&
    movie.user_data.position < 0.9 * movie.video_info.duration
  );
}
