import 'dart:convert';

import 'package:dio_image_provider/dio_image_provider.dart';
import 'package:flutter/material.dart';

final _transparentImage = base64Decode(
    'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=');

class ZenithFadeInImage extends StatelessWidget {
  final ImageProvider<Object> image;
  final double? width;
  final double? height;
  final BoxFit fit;
  final AlignmentGeometry alignment;

  const ZenithFadeInImage({
    super.key,
    required this.image,
    this.width,
    this.height,
    this.fit = BoxFit.cover,
    this.alignment = Alignment.center,
  });

  ZenithFadeInImage.dio({
    super.key,
    required String url,
    this.width,
    this.height,
    this.fit = BoxFit.cover,
    this.alignment = Alignment.center,
  }) : image = DioImage.string(url);

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
