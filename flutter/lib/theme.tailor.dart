// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, unused_element

part of 'theme.dart';

// **************************************************************************
// ThemeTailorGenerator
// **************************************************************************

class ZenithTheme extends ThemeExtension<ZenithTheme> {
  const ZenithTheme({
    required this.titleMedium,
    required this.bodyMedium,
    required this.bodySmall,
  });

  final TextStyle titleMedium;
  final TextStyle bodyMedium;
  final TextStyle bodySmall;

  static final ZenithTheme light = ZenithTheme(
    titleMedium: _$ZenithTheme.titleMedium[0],
    bodyMedium: _$ZenithTheme.bodyMedium[0],
    bodySmall: _$ZenithTheme.bodySmall[0],
  );

  static final ZenithTheme dark = ZenithTheme(
    titleMedium: _$ZenithTheme.titleMedium[1],
    bodyMedium: _$ZenithTheme.bodyMedium[1],
    bodySmall: _$ZenithTheme.bodySmall[1],
  );

  static final themes = [
    light,
    dark,
  ];

  @override
  ZenithTheme copyWith({
    TextStyle? titleMedium,
    TextStyle? bodyMedium,
    TextStyle? bodySmall,
  }) {
    return ZenithTheme(
      titleMedium: titleMedium ?? this.titleMedium,
      bodyMedium: bodyMedium ?? this.bodyMedium,
      bodySmall: bodySmall ?? this.bodySmall,
    );
  }

  @override
  ZenithTheme lerp(ThemeExtension<ZenithTheme>? other, double t) {
    if (other is! ZenithTheme) return this;
    return ZenithTheme(
      titleMedium: TextStyle.lerp(titleMedium, other.titleMedium, t)!,
      bodyMedium: TextStyle.lerp(bodyMedium, other.bodyMedium, t)!,
      bodySmall: TextStyle.lerp(bodySmall, other.bodySmall, t)!,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is ZenithTheme &&
            const DeepCollectionEquality()
                .equals(titleMedium, other.titleMedium) &&
            const DeepCollectionEquality()
                .equals(bodyMedium, other.bodyMedium) &&
            const DeepCollectionEquality().equals(bodySmall, other.bodySmall));
  }

  @override
  int get hashCode {
    return Object.hash(
        runtimeType,
        const DeepCollectionEquality().hash(titleMedium),
        const DeepCollectionEquality().hash(bodyMedium),
        const DeepCollectionEquality().hash(bodySmall));
  }
}

extension ZenithThemeBuildContext on BuildContext {
  ZenithTheme get zenithTheme => Theme.of(this).extension<ZenithTheme>()!;
}
