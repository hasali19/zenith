import 'package:flutter/widgets.dart';
import 'package:sized_context/sized_context.dart';

extension MediaQueryDataExt on MediaQueryData {
  bool get isDesktop => size.width > 960;
}

extension BuildContextExt on BuildContext {
  bool get isDesktop => mq.size.width > 960;
}
