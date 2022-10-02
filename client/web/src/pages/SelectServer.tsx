import { Component, createSignal, Show } from "solid-js";
import preferences from "../preferences";

export const SelectServerScreen: Component = () => {
  const [server, setServer] = createSignal("");
  const [isLoading, setLoading] = createSignal(false);
  const [isError, setError] = createSignal(false);

  const isValid = () => {
    return /^https?:\/\/\S+$/.test(server());
  };

  const onSubmit = async (e: Event) => {
    e.preventDefault();
    const address = server();
    setError(false);
    setLoading(true);
    try {
      // TODO: Add a ping endpoint
      const res = await fetch(`${address}/api/movies`);
      if (res.ok) {
        preferences.server = address;
      } else {
        setError(true);
      }
    } catch {
      setError(true);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div class="flex-1 flex items-center justify-center">
      <form class="flex-1 max-w-[480px] flex flex-col" onSubmit={onSubmit}>
        <h1 class="my-4 text-center">Select server</h1>
        <input
          type="url"
          class="bg-white/10 rounded-md text-white focus-visible:bg-[rgba(50,50,50,0.6)] placeholder:text-neutral-300"
          placeholder="Server address"
          onInput={(e) => setServer(e.currentTarget.value)}
        />
        <button
          class="mt-4 disabled:bg-[#d16318]"
          classList={{ "is-loading": isLoading() }}
          disabled={isLoading() || !isValid()}
        >
          Connect
        </button>
      </form>
      <Show when={isError()}>
        <div
          class="notification is-danger"
          style={{ position: "fixed", bottom: "16px", left: "16px" }}
        >
          <button class="delete" onClick={() => setError(false)}></button>
          <span class="icon">
            <span class="mdi mdi-18px mdi-wifi"></span>
          </span>
          <span style={{ "margin-left": "8px" }}>
            Couldn't connect to server
          </span>
        </div>
      </Show>
    </div>
  );
};
