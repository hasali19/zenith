import 'package:flutter/material.dart';
import 'package:theme_tailor_annotation/theme_tailor_annotation.dart';

part 'theme.tailor.dart';

@Tailor(themeGetter: ThemeGetter.onBuildContext)
class _$ZenithTheme {
  static List<TextStyle> titleLarge = [];
  static List<TextStyle> titleMedium = [];
  static List<TextStyle> bodyMedium = [];
  static List<TextStyle> bodySmall = [];
}
