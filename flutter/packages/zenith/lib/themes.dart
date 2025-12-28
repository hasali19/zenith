import 'package:flutter/material.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';

part 'themes.g.dart';

class Themes {
  final ThemeData light;
  final ThemeData dark;
  const Themes(this.light, this.dark);
}

@Riverpod(dependencies: [])
Themes themes(Ref ref) {
  throw UnimplementedError();
}
