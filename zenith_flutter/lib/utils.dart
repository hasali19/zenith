String formatDuration(double duration) {
  if (duration <= 90 * 60) {
    return '${(duration / 60).floor()}m';
  } else {
    final hours = (duration / 3600).floor();
    final minutes = ((duration % 3600) / 60).floor();
    return '${hours}h ${minutes}m';
  }
}
