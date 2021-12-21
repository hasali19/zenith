import { Component, createSignal, onMount, Show } from "solid-js";
import { CastMediaPlayer } from "./CastMediaPlayer";

import * as styles from "./App.css";
import splash from "./assets/zenith_full.png";

interface MovieDetails {
  movie: any;
}

interface ShowDetails {
  show: any;
}

interface SeasonDetails {
  show: any;
  season: any;
}

interface EpisodeDetails {
  show: any;
  season: any;
  episode: any;
}

type MediaItemDetails =
  | MovieDetails
  | ShowDetails
  | SeasonDetails
  | EpisodeDetails;

const App: Component = () => {
  const [state, setState] = createSignal(cast.framework.ui.State.LAUNCHING);
  const [activeItem, setActiveItem] = createSignal<MediaItemDetails | null>(
    null
  );

  onMount(() => {
    const options = new cast.framework.CastReceiverOptions();

    if ((import.meta as any).env.DEV) {
      options.disableIdleTimeout = true;
    }

    const context = cast.framework.CastReceiverContext.getInstance();
    const playerManager = context.getPlayerManager();

    playerManager.addEventListener(
      cast.framework.events.EventType.PLAYER_LOAD_COMPLETE,
      () => {
        const textTracksManager = playerManager.getTextTracksManager();

        // Set default subtitle style
        const style = new cast.framework.messages.TextTrackStyle();
        style.backgroundColor = "#00000000";
        style.edgeColor = "#000000ff";
        style.edgeType = cast.framework.messages.TextTrackEdgeType.OUTLINE;
        style.fontFamily = "sans-serif";
        style.fontScale = 0.8;
        textTracksManager.setTextTrackStyle(style);

        // Turn off subtitles by default
        textTracksManager.setActiveByIds([]);
      }
    );

    const playerData = {};
    const playerDataBinder = new cast.framework.ui.PlayerDataBinder(playerData);

    playerDataBinder.addEventListener(
      cast.framework.ui.PlayerDataEventType.STATE_CHANGED,
      (e) => {
        setState(e.value);
      }
    );

    context.addCustomMessageListener(
      "urn:x-cast:uk.hasali.zenith.cast",
      (e) => {
        setActiveItem(e.data);
      }
    );

    context.start(options);
  });

  const showUi = () =>
    state() == cast.framework.ui.State.LAUNCHING ||
    state() == cast.framework.ui.State.IDLE;

  return (
    <>
      <CastMediaPlayer />
      <Show when={showUi()}>
        <div class={styles.idleUiContainer}>
          <Show when={activeItem()} fallback={<SplashScreen />}>
            {renderItemDetails}
          </Show>
        </div>
      </Show>
    </>
  );
};

export default App;

const SplashScreen: Component = () => {
  const width = 600 / devicePixelRatio;
  const height = 229 / devicePixelRatio;
  return (
    <div class={styles.splashScreen}>
      <img
        src={splash}
        style={{ width: `${width}px`, height: `${height}px` }}
      />
    </div>
  );
};

function renderItemDetails(item: MediaItemDetails) {
  let image!: { type: "poster" | "still"; src: string | null };
  let backdrop!: string | null;
  let pretitle!: string | undefined;
  let title!: string;
  let subtitle: any | undefined;
  let overview!: string | null;

  if ("movie" in item) {
    image = { type: "poster", src: item.movie.poster };
    backdrop = item.movie.backdrop;
    title = item.movie.title;
    subtitle = (
      <>
        <span>{formatYear(item.movie.release_date)}</span>
        <span style={{ margin: "0px 8px" }}>·</span>
        <span>{formatDuration(item.movie.video_info.duration)}</span>
      </>
    );
    overview = item.movie.overview;
  } else if ("episode" in item) {
    image = { type: "still", src: item.episode.thumbnail };
    backdrop = item.season.backdrop;
    pretitle = item.show.name;
    title = item.episode.name;
    subtitle = (
      <>
        <span>S{item.episode.season_number.toString().padStart(2, "0")}</span>
        <span>E{item.episode.episode_number.toString().padStart(2, "0")}</span>
        <span style={{ margin: "0px 16px" }}>·</span>
        <span>{formatDuration(item.episode.video_info.duration)}</span>
      </>
    );
    overview = item.episode.overview;
  } else if ("season" in item) {
    image = { type: "poster", src: item.season.poster };
    backdrop = item.season.backdrop;
    pretitle = item.show.name;
    title = item.season.name ?? `Season ${item.season.season_number}`;
    overview = item.season.overview;
  } else if ("show" in item) {
    image = { type: "poster", src: item.show.poster };
    backdrop = item.show.backdrop;
    title = item.show.name;
    subtitle = formatYear(item.show.start_date);
    overview = item.show.overview;
  }

  return (
    <ItemDetails backdrop={backdrop} image={image}>
      <h3 class={styles.pretitle}>{pretitle}</h3>
      <h1 class={styles.title}>{title}</h1>
      <h2 class={styles.subtitle}>{subtitle}</h2>
      <p class={styles.overview}>{overview}</p>
    </ItemDetails>
  );
}

interface ItemDetailsProps {
  backdrop: string | null;
  image: {
    type: "poster" | "still";
    src: string | null;
  };
}

const ItemDetails: Component<ItemDetailsProps> = (p) => {
  return (
    <>
      <div
        class={styles.backdrop}
        style={{ "background-image": `url(${p.backdrop})` }}
      >
        <div class={styles.backdropOverlay}></div>
      </div>
      <div class={styles.item}>
        <div>
          <Show when={p.image.src}>
            {(src) => (
              <img
                src={src}
                class={
                  { poster: styles.poster, still: styles.still }[p.image.type]
                }
              ></img>
            )}
          </Show>
        </div>
        <div class={styles.content}>{p.children}</div>
      </div>
    </>
  );
};

function formatYear(timestamp: number) {
  if (!timestamp) return;
  return new Date(timestamp * 1000).getFullYear();
}

function formatDuration(duration: number): string {
  if (duration <= 90 * 60) {
    return `${Math.floor(duration / 60)}m`;
  } else {
    const hours = Math.floor(duration / 3600);
    const minutes = Math.floor((duration % 3600) / 60);
    return `${hours}h ${minutes}m`;
  }
}
