// import Vue from "vue";

import { createContext, FC, useContext, useEffect, useState } from "react";

const CAST_SENDER_SCRIPT_SRC =
  "https://www.gstatic.com/cv/js/sender/v1/cast_sender.js?loadCastFramework=1";

async function connect() {
  const context = cast.framework.CastContext.getInstance();
  return new Promise<cast.framework.CastSession>((resolve, reject) => {
    context.requestSession().then(
      () => {
        const session = context.getCurrentSession();
        if (session) {
          resolve(session);
        } else {
          reject("session not found");
        }
      },
      () => reject("failed to create session")
    );
  });
}

const Context = createContext({
  ready: false,
  connected: false,
  paused: false,
  currentTime: 0,
  connect,
});

export const GCastProvider: FC = ({ children }) => {
  const [ready, setReady] = useState(false);
  const [connected, setConnected] = useState(false);
  const [paused, setPaused] = useState(false);
  const [currentTime, setCurrentTime] = useState(0);

  useEffect(() => {
    window.__onGCastApiAvailable = function (isAvailable: boolean) {
      if (!isAvailable) {
        console.warn("cast api not available");
        return;
      }

      cast.framework.CastContext.getInstance().setOptions({
        receiverApplicationId: chrome.cast.media.DEFAULT_MEDIA_RECEIVER_APP_ID,
        autoJoinPolicy: chrome.cast.AutoJoinPolicy.ORIGIN_SCOPED,
      });

      const player = new cast.framework.RemotePlayer();
      const controller = new cast.framework.RemotePlayerController(player);

      setReady(isAvailable);
      setConnected(player.isConnected);

      controller.addEventListener(
        cast.framework.RemotePlayerEventType.IS_CONNECTED_CHANGED,
        () => {
          setConnected(player.isConnected);
        }
      );

      controller.addEventListener(
        cast.framework.RemotePlayerEventType.IS_PAUSED_CHANGED,
        () => {
          setPaused(player.isPaused);
        }
      );

      controller.addEventListener(
        cast.framework.RemotePlayerEventType.CURRENT_TIME_CHANGED,
        () => {
          setCurrentTime(player.currentTime);
        }
      );
    };

    const script = document.createElement("script");
    script.src = CAST_SENDER_SCRIPT_SRC;
    document.head.appendChild(script);
  }, []);

  const gcast = {
    ready,
    connected,
    paused,
    currentTime,
    connect,
  };

  return <Context.Provider value={gcast}>{children}</Context.Provider>;
};

export function useGCast() {
  return useContext(Context);
}
