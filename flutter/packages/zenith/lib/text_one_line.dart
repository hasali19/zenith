import 'package:flutter/material.dart';

////////////////////////////////////////////////////////////////////////////////////////////////////

/// [TextOneLine] is a substitute for [Text] when [maxLines] is 1.
///
/// It renders ellipsis as expected, much better than the current
/// buggy and ugly-looking ellipsis of the native [Text].
///
/// This widget only makes sense while that issue is not fixed:
/// https://github.com/flutter/flutter/issues/18761
///
///
class TextOneLine extends Text {
  const TextOneLine(
    super.data, {
    super.key,
    super.style,
    super.strutStyle,
    super.textAlign,
    super.textDirection,
    super.locale,
    super.overflow = TextOverflow.fade,
    super.semanticsLabel,
    TextWidthBasis super.textWidthBasis = TextWidthBasis.parent,
    super.textHeightBehavior,
  }) : super(
          softWrap: false,
          maxLines: 1,
        );
}
