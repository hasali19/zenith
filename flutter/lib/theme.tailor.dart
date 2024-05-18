// coverage:ignore-file
// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, unused_element, unnecessary_cast

part of 'theme.dart';

// **************************************************************************
// TailorAnnotationsGenerator
// **************************************************************************

mixin _$ZenithThemeTailorMixin on ThemeExtension<ZenithTheme> {
  TextStyle get titleLarge;
  TextStyle get titleMedium;
  TextStyle get bodyMedium;
  TextStyle get bodySmall;

  @override
  ZenithTheme copyWith({
    TextStyle? titleLarge,
    TextStyle? titleMedium,
    TextStyle? bodyMedium,
    TextStyle? bodySmall,
  }) {
    return ZenithTheme(
      titleLarge: titleLarge ?? this.titleLarge,
      titleMedium: titleMedium ?? this.titleMedium,
      bodyMedium: bodyMedium ?? this.bodyMedium,
      bodySmall: bodySmall ?? this.bodySmall,
    );
  }

  @override
  ZenithTheme lerp(covariant ThemeExtension<ZenithTheme>? other, double t) {
    if (other is! ZenithTheme) return this as ZenithTheme;
    return ZenithTheme(
      titleLarge: TextStyle.lerp(titleLarge, other.titleLarge, t)!,
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
                .equals(titleLarge, other.titleLarge) &&
            const DeepCollectionEquality()
                .equals(titleMedium, other.titleMedium) &&
            const DeepCollectionEquality()
                .equals(bodyMedium, other.bodyMedium) &&
            const DeepCollectionEquality().equals(bodySmall, other.bodySmall));
  }

  @override
  int get hashCode {
    return Object.hash(
      runtimeType.hashCode,
      const DeepCollectionEquality().hash(titleLarge),
      const DeepCollectionEquality().hash(titleMedium),
      const DeepCollectionEquality().hash(bodyMedium),
      const DeepCollectionEquality().hash(bodySmall),
    );
  }
}

extension ZenithThemeBuildContext on BuildContext {
  ZenithTheme get zenithTheme => Theme.of(this).extension<ZenithTheme>()!;
}
