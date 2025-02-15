import { createEffect, createSignal, onCleanup, onMount, Show } from "solid-js";
import { isDev } from "solid-js/web";
import { CastMediaPlayer } from "./CastMediaPlayer";

const MESSAGE_CHANNEL = "urn:x-cast:dev.hasali.zenith";

enum MessageType {
  INIT = "init",
  FOCUS_ITEM_DETAILS = "focus-item-details",
}

type CustomMessage = InitMessage | FocusItemDetailsMessage | UnknownMessage;

interface InitMessage {
  type: MessageType.INIT;
  server: string;
  token: string;
}

interface FocusItemDetailsMessage {
  type: MessageType.FOCUS_ITEM_DETAILS;
  id: number;
}

interface UnknownMessage {
  type: never;
}

interface ServerConfig {
  baseUrl: string;
  token: string;
}

declare module "chromecast-caf-receiver/cast.framework.messages" {
  interface QueueItemCustomData {
    id?: number;
  }
}

declare module "chromecast-caf-receiver/cast.framework.system" {
  interface Event {
    senderId: string;
  }
}

export default function App() {
  const [config, setConfig] = createSignal<ServerConfig | null>(null);
  const [isPlaybackActive, setPlaybackActive] = createSignal(false);
  const [focusedItemId, setFocusedItemId] = createSignal<number | null>(null);

  onMount(async () => {
    const context = cast.framework.CastReceiverContext.getInstance();
    const playerManager = context.getPlayerManager();

    let server: string | null = null;
    let token: string | null = null;

    context.addCustomMessageListener(MESSAGE_CHANNEL, (e) => {
      const sender = e.senderId;
      const msg = e.data as CustomMessage | null;

      if (!msg) {
        return;
      }

      switch (msg.type) {
        case MessageType.INIT: {
          setConfig({ baseUrl: msg.server, token: msg.token });
          context.sendCustomMessage(MESSAGE_CHANNEL, sender, {
            type: MessageType.INIT,
          });
          break;
        }
        case MessageType.FOCUS_ITEM_DETAILS: {
          setFocusedItemId(msg.id);
          break;
        }
      }
    });

    playerManager.addEventListener(
      cast.framework.events.EventType.REQUEST_LOAD,
      () => {
        setPlaybackActive(true);
      }
    );

    playerManager.addEventListener(
      cast.framework.events.EventType.MEDIA_STATUS,
      (e) => {
        const state = e.mediaStatus?.playerState;
        if (!state) {
          return;
        }

        setPlaybackActive(state !== cast.framework.messages.PlayerState.IDLE);
      }
    );

    const timer = setInterval(() => {
      const state = playerManager.getPlayerState();
      const currentItem = playerManager.getQueueManager()?.getCurrentItem();

      if (
        server &&
        token &&
        state === cast.framework.messages.PlayerState.PLAYING &&
        currentItem
      ) {
        const videoId = currentItem.customData?.id;
        if (videoId) {
          const currentTime = playerManager.getCurrentTimeSec();
          if (isDev) {
            console.log(
              `Updating progress for video=${videoId}, position=${currentTime}`
            );
          }
          const url = `${server}/api/progress/${videoId}?position=${currentTime}&token=${encodeURIComponent(
            token
          )}`;
          fetch(url, { method: "POST" });
        }
      }
    }, 5000);

    onCleanup(() => {
      clearInterval(timer);
    });

    context.start();
  });

  const [item, setItem] = createSignal<any>();

  const token = () => {
    const c = config();
    if (c) {
      return encodeURIComponent(c.token);
    }
  };

  createEffect(async () => {
    const serverConfig = config();
    if (!serverConfig) {
      return;
    }

    const id = focusedItemId();
    if (!id) {
      return;
    }

    const url = `${serverConfig.baseUrl}/api/items/${id}?token=${token()}`;
    const res = await fetch(url);
    const json = await res.json();

    setItem(json);
  });

  const imageUrl = (id: number, width: number) =>
    `${config()?.baseUrl}/api/images/${id}?token=${token()}&width=${width}`;

  const backdrop = () => imageUrl(item().backdrop, 1280);
  const poster = () => imageUrl(item().poster, 342);

  const title = () => {
    switch (item().type) {
      case "movie":
      case "show":
        return item().name;
      case "episode":
        return item().grandparent.name;
    }
  };

  const subtitle = () => {
    switch (item().type) {
      case "movie":
      case "show":
        return new Date(item().start_date * 1000).getFullYear();
      case "episode": {
        const season = item().grandparent.index.toString().padStart(2, "0");
        const episode = item().parent.index.toString().padStart(2, "0");
        return `S${season}E${episode}: ${item().name}`;
      }
    }
  };

  return (
    <>
      <CastMediaPlayer />
      <Show when={!isPlaybackActive() && item()}>
        <div
          style={{
            position: "absolute",
            width: "100vw",
            height: "100vh",
            filter: "blur(8px)",
          }}
        >
          <img
            src={backdrop()}
            style={{
              position: "absolute",
              width: "100vw",
              height: "100vh",
              transition: "opacity 300ms ease",
              opacity: "0",
              "object-position": "center",
              "object-fit": "cover",
            }}
            onload={(e) => {
              e.target.style.opacity = "1";
            }}
          />
        </div>
        <div
          style={{
            position: "absolute",
            top: 0,
            bottom: 0,
            left: 0,
            right: 0,
            display: "flex",
            "align-items": "center",
            "background-color": "rgba(0, 0, 0, 0.3)",
            color: "white",
            padding: "64px",
            "font-family": "'Exo 2', sans-serif",
          }}
        >
          <img
            src={poster()}
            style={{
              width: "200px",
              height: "300px",
              "border-radius": "16px",
              opacity: 0,
              transition: "opacity 300ms ease",
            }}
            onload={(e) => {
              e.target.style.opacity = "1";
            }}
          />
          <div style={{ "margin-left": "32px" }}>
            <h1 style={{ margin: 0 }}>{title()}</h1>
            <h3 style={{ margin: 0 }}>{subtitle()}</h3>
            <p>{item().overview}</p>
          </div>
        </div>
        <img
          src="/chromecast-receiver/zenith_full_dark.png"
          style={{
            position: "absolute",
            top: 0,
            right: 0,
            width: "100px",
            margin: "32px",
          }}
        />
      </Show>
    </>
  );
}
