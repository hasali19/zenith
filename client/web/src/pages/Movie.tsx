import { useNavigate, useParams } from "solid-app-router";
import { Component, createEffect, createSignal, Show } from "solid-js";
import { MediaDetailsScreen } from "../MediaDetailsScreen";
import preferences from "../preferences";
import { formatYear } from "../utils";

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
          poster={movie.poster}
          backdrop={movie.backdrop}
          subtitle={formatYear(movie.release_date)}
          overview={movie.overview}
          tmdbLink={`https://www.themoviedb.org/movie/${movie.external_ids.tmdb}`}
          headerActions={
            <>
              <button
                class="button is-primary"
                onClick={() => navigate(`/player/${params.id}`)}
              >
                <span class="icon">
                  <span class="material-icons">play_arrow</span>
                </span>
                <span>Play</span>
              </button>
            </>
          }
        />
      )}
    </Show>
  );
};
