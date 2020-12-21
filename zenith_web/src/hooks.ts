import { useEffect } from "react";

export function useOnce(fn: React.EffectCallback) {
  // eslint-disable-next-line
  useEffect(fn, []);
}
