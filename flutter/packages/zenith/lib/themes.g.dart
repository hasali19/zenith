// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'themes.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(themes)
final themesProvider = ThemesProvider._();

final class ThemesProvider extends $FunctionalProvider<Themes, Themes, Themes>
    with $Provider<Themes> {
  ThemesProvider._()
      : super(
          from: null,
          argument: null,
          retry: null,
          name: r'themesProvider',
          isAutoDispose: true,
          dependencies: <ProviderOrFamily>[],
          $allTransitiveDependencies: <ProviderOrFamily>[],
        );

  @override
  String debugGetCreateSourceHash() => _$themesHash();

  @$internal
  @override
  $ProviderElement<Themes> $createElement($ProviderPointer pointer) =>
      $ProviderElement(pointer);

  @override
  Themes create(Ref ref) {
    return themes(ref);
  }

  /// {@macro riverpod.override_with_value}
  Override overrideWithValue(Themes value) {
    return $ProviderOverride(
      origin: this,
      providerOverride: $SyncValueProvider<Themes>(value),
    );
  }
}

String _$themesHash() => r'2b3fa2a6f03fde0b33d021bab79ace02894dc521';
