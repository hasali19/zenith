import 'dart:convert';

import 'package:dio_image_provider/dio_image_provider.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/api.dart';

final _transparentImage = base64Decode(
    'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=');

class ZenithApiImage extends ConsumerWidget {
  final ImageId id;
  final int? requestWidth;
  final double? width;
  final double? height;
  final BoxFit fit;
  final AlignmentGeometry alignment;

  const ZenithApiImage({
    super.key,
    required this.id,
    required this.requestWidth,
    this.width,
    this.height,
    this.fit = BoxFit.cover,
    this.alignment = Alignment.center,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final api = ref.watch(apiProvider);
    return ZenithImage(
      image: DioImage.string(api.getImageUrl(id, width: requestWidth)),
      width: width,
      height: height,
      fit: fit,
      alignment: alignment,
    );
  }
}

class ZenithImage extends StatelessWidget {
  final ImageProvider image;
  final double? width;
  final double? height;
  final BoxFit fit;
  final AlignmentGeometry alignment;

  const ZenithImage({
    super.key,
    required this.image,
    this.width,
    this.height,
    this.fit = BoxFit.cover,
    this.alignment = Alignment.center,
  });

  @override
  Widget build(BuildContext context) {
    return FadeInImage(
      placeholder: MemoryImage(_transparentImage),
      image: image,
      width: width,
      height: height,
      fit: fit,
      alignment: alignment,
    );
  }
}
