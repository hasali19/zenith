import 'dart:math';

import 'package:web/web.dart';

final cueRegex =
    RegExp(r'(?:\d+\n)?([\d\.,:]+)\s*-->\s*([\d\.,:]+)\r?\n', multiLine: true);

abstract class TextTrackParser {
  List<TextTrackCue> parse(String data);
}

double _parseTime(String time) {
  final segments = time.split(':');
  double seconds = 0;
  for (var i = 0; i < segments.length; i++) {
    final segment = segments[i];
    final value = double.parse(segment.replaceFirst(',', '.'));
    seconds += value * pow(60, segments.length - i - 1);
  }
  return seconds;
}

class SrtParser extends TextTrackParser {
  @override
  List<TextTrackCue> parse(String data) {
    final sections = data.split('\n\n');
    final cues = <TextTrackCue>[];

    for (final section in sections) {
      final match = cueRegex.firstMatch(section);
      if (match != null) {
        cues.add(VTTCue(_parseTime(match.group(1)!),
            _parseTime(match.group(2)!), section.substring(match.end)));
      }
    }

    return cues;
  }
}

class VttParser extends TextTrackParser {
  @override
  List<TextTrackCue> parse(String data) {
    final index = data.indexOf(RegExp(r'(?:\r?\n){2}'));
    return SrtParser().parse(data.substring(index + 2));
  }
}
