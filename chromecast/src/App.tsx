import { onCleanup, onMount } from "solid-js";
import { isDev } from "solid-js/web";
import { CastMediaPlayer } from "./CastMediaPlayer";

const MESSAGE_CHANNEL = "urn:x-cast:dev.hasali.zenith";

enum MessageType {
  INIT = "init",
}

type CustomMessage = InitMessage | UnknownMessage;

interface InitMessage {
  type: MessageType.INIT;
  server: string;
  token: string;
}

interface UnknownMessage {
  type: never;
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
  onMount(async () => {
    const context = cast.framework.CastReceiverContext.getInstance();
    const playerManager = context.getPlayerManager();

    let server: string | null = null;
    let token: string | null = null;

    context.addCustomMessageListener(MESSAGE_CHANNEL, (e) => {
      const sender = e.senderId;
      const msg = e.data as CustomMessage | null;
      if (msg && msg.type === MessageType.INIT) {
        server = msg.server;
        token = msg.token;
        context.sendCustomMessage(MESSAGE_CHANNEL, sender, {
          type: MessageType.INIT,
        });
      }
    });

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

  return (
    <>
      <CastMediaPlayer />
    </>
  );
}
