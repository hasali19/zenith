import 'package:flutter/material.dart';
import 'package:theme_tailor_annotation/theme_tailor_annotation.dart';

part 'theme.tailor.dart';

@TailorMixin(themeGetter: ThemeGetter.onBuildContext)
class ZenithTheme extends ThemeExtension<ZenithTheme>
    with _$ZenithThemeTailorMixin {
  @override
  final TextStyle titleLarge;
  @override
  final TextStyle titleMedium;
  @override
  final TextStyle bodyMedium;
  @override
  final TextStyle bodySmall;

  const ZenithTheme({
    required this.titleLarge,
    required this.titleMedium,
    required this.bodyMedium,
    required this.bodySmall,
  });
}
