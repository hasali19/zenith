// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, unused_element, unnecessary_cast

part of 'theme.dart';

// **************************************************************************
// TailorAnnotationsGenerator
// **************************************************************************

class ZenithTheme extends ThemeExtension<ZenithTheme> {
  const ZenithTheme({
    required this.bodyMedium,
    required this.bodySmall,
    required this.titleLarge,
    required this.titleMedium,
  });

  final TextStyle bodyMedium;
  final TextStyle bodySmall;
  final TextStyle titleLarge;
  final TextStyle titleMedium;

  static final ZenithTheme light = ZenithTheme(
    bodyMedium: _$ZenithTheme.bodyMedium[0],
    bodySmall: _$ZenithTheme.bodySmall[0],
    titleLarge: _$ZenithTheme.titleLarge[0],
    titleMedium: _$ZenithTheme.titleMedium[0],
  );

  static final ZenithTheme dark = ZenithTheme(
    bodyMedium: _$ZenithTheme.bodyMedium[1],
    bodySmall: _$ZenithTheme.bodySmall[1],
    titleLarge: _$ZenithTheme.titleLarge[1],
    titleMedium: _$ZenithTheme.titleMedium[1],
  );

  static final themes = [
    light,
    dark,
  ];

  @override
  ZenithTheme copyWith({
    TextStyle? bodyMedium,
    TextStyle? bodySmall,
    TextStyle? titleLarge,
    TextStyle? titleMedium,
  }) {
    return ZenithTheme(
      bodyMedium: bodyMedium ?? this.bodyMedium,
      bodySmall: bodySmall ?? this.bodySmall,
      titleLarge: titleLarge ?? this.titleLarge,
      titleMedium: titleMedium ?? this.titleMedium,
    );
  }

  @override
  ZenithTheme lerp(covariant ThemeExtension<ZenithTheme>? other, double t) {
    if (other is! ZenithTheme) return this as ZenithTheme;
    return ZenithTheme(
      bodyMedium: TextStyle.lerp(bodyMedium, other.bodyMedium, t)!,
      bodySmall: TextStyle.lerp(bodySmall, other.bodySmall, t)!,
      titleLarge: TextStyle.lerp(titleLarge, other.titleLarge, t)!,
      titleMedium: TextStyle.lerp(titleMedium, other.titleMedium, t)!,
    );
  }

  @override
  bool operator ==(Object other) {
    return identical(this, other) ||
        (other.runtimeType == runtimeType &&
            other is ZenithTheme &&
            const DeepCollectionEquality()
                .equals(bodyMedium, other.bodyMedium) &&
            const DeepCollectionEquality().equals(bodySmall, other.bodySmall) &&
            const DeepCollectionEquality()
                .equals(titleLarge, other.titleLarge) &&
            const DeepCollectionEquality()
                .equals(titleMedium, other.titleMedium));
  }

  @override
  int get hashCode {
    return Object.hash(
      runtimeType.hashCode,
      const DeepCollectionEquality().hash(bodyMedium),
      const DeepCollectionEquality().hash(bodySmall),
      const DeepCollectionEquality().hash(titleLarge),
      const DeepCollectionEquality().hash(titleMedium),
    );
  }
}

extension ZenithThemeBuildContext on BuildContext {
  ZenithTheme get zenithTheme => Theme.of(this).extension<ZenithTheme>()!;
}
