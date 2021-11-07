export function displayYear(timestamp: number | null): string | undefined {
  if (!timestamp) return;
  return new Date(timestamp * 1000).getUTCFullYear().toString();
}

export function displayDuration(duration: number): string {
  if (duration <= 90 * 60) {
    return `${Math.floor(duration / 60)}m`;
  } else {
    const hours = Math.floor(duration / 3600);
    const minutes = Math.floor((duration % 3600) / 60);
    return `${hours}h ${minutes}m`;
  }
}
