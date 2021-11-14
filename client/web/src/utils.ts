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
