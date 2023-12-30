import { onMount } from "solid-js";
import { CastMediaPlayer } from "./CastMediaPlayer";

export default function App() {
  onMount(async () => {
    const context = cast.framework.CastReceiverContext.getInstance();
    context.start();
  });

  return (
    <>
      <CastMediaPlayer />
    </>
  );
}
