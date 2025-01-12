import { onCleanup, onMount } from "solid-js";
import { isDev } from "solid-js/web";
import { CastMediaPlayer } from "./CastMediaPlayer";

const MESSAGE_CHANNEL = "urn:x-cast:dev.hasali.zenith";

declare module "chromecast-caf-receiver/cast.framework.messages" {
  interface QueueItemCustomData {
    id?: number;
  }
}

export default function App() {
  onMount(async () => {
    const context = cast.framework.CastReceiverContext.getInstance();
    const playerManager = context.getPlayerManager();

    let server: string | null = null;
    let token: string | null = null;

    context.addCustomMessageListener(MESSAGE_CHANNEL, (e) => {
      const msg = e.data as any;
      if (msg && msg.server && msg.token) {
        server = msg.server;
        token = msg.token;
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
