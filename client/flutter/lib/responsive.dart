import 'package:flutter/widgets.dart';

extension MediaQueryDataExt on MediaQueryData {
  bool get isDesktop => size.width > 960;
}
