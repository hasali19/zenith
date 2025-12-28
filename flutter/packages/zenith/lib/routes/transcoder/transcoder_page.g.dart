// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'transcoder_page.dart';

// **************************************************************************
// RiverpodGenerator
// **************************************************************************

// GENERATED CODE - DO NOT MODIFY BY HAND
// ignore_for_file: type=lint, type=warning

@ProviderFor(_data)
final _dataProvider = _DataProvider._();

final class _DataProvider extends $FunctionalProvider<
        AsyncValue<
            List<
                (
                  TranscoderJob,
                  MediaItem,
                )>>,
        List<
            (
              TranscoderJob,
              MediaItem,
            )>,
        FutureOr<
            List<
                (
                  TranscoderJob,
                  MediaItem,
                )>>>
    with
        $FutureModifier<
            List<
                (
                  TranscoderJob,
                  MediaItem,
                )>>,
        $FutureProvider<
            List<
                (
                  TranscoderJob,
                  MediaItem,
                )>> {
  _DataProvider._()
      : super(
          from: null,
          argument: null,
          retry: null,
          name: r'_dataProvider',
          isAutoDispose: true,
          dependencies: null,
          $allTransitiveDependencies: null,
        );

  @override
  String debugGetCreateSourceHash() => _$_dataHash();

  @$internal
  @override
  $FutureProviderElement<
      List<
          (
            TranscoderJob,
            MediaItem,
          )>> $createElement($ProviderPointer pointer) =>
      $FutureProviderElement(pointer);

  @override
  FutureOr<
      List<
          (
            TranscoderJob,
            MediaItem,
          )>> create(Ref ref) {
    return _data(ref);
  }
}

String _$_dataHash() => r'6c30cc962a0d091c4559d83619dd82c1cfa2b30a';
