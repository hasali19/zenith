export function formatYear(timestamp: number) {
  if (!timestamp) return;
  return new Date(timestamp * 1000).getFullYear();
}

export function formatDuration(duration: number): string {
  if (duration <= 90 * 60) {
    return `${Math.floor(duration / 60)}m`;
  } else {
    const hours = Math.floor(duration / 3600);
    const minutes = Math.floor((duration % 3600) / 60);
    return `${hours}h ${minutes}m`;
  }
}

export function formatPosition(position: number, duration: number) {
  const hours = Math.floor(position / 3600);
  const minutes = Math.floor((position % 3600) / 60);
  const seconds = Math.floor(position % 60);
  const fmt = (v: number) => v.toString().padStart(2, "0");
  if (duration > 3600) {
    return `${fmt(hours)}:${fmt(minutes)}:${fmt(seconds)}`;
  } else {
    return `${fmt(minutes)}:${fmt(seconds)}`;
  }
}
