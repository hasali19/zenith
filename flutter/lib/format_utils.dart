String formatSeason(int seasonNumber) {
  final season = seasonNumber.toString().padLeft(2, '0');
  return 'S$season';
}

String formatSeasonEpisode(int seasonNumber, int episodeNumber) {
  final season = seasonNumber.toString().padLeft(2, '0');
  final episode = episodeNumber.toString().padLeft(2, '0');
  return 'S${season}E$episode';
}
