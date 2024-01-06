import 'dart:convert';

import 'package:dio_image_provider/dio_image_provider.dart';
import 'package:flutter/material.dart';

final _transparentImage = base64Decode(
    'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=');

class ZenithFadeInImage extends StatelessWidget {
  final ImageProvider<Object> image;
  final BoxFit fit;

  const ZenithFadeInImage({
    super.key,
    required this.image,
    this.fit = BoxFit.cover,
  });

  ZenithFadeInImage.dio({
    super.key,
    required String url,
    this.fit = BoxFit.cover,
  }) : image = DioImage.string(url);

  @override
  Widget build(BuildContext context) {
    return FadeInImage(
      placeholder: MemoryImage(_transparentImage),
      image: image,
      fit: fit,
    );
  }
}
