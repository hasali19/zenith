import 'package:flutter/foundation.dart';
import 'package:flutter/widgets.dart';
import 'package:zenith/router/router_delegate.dart';

class ZenithRouteInformationParser
    extends RouteInformationParser<RouteLocation> {
  @override
  Future<RouteLocation> parseRouteInformation(
      RouteInformation routeInformation) {
    final uri = routeInformation.uri;
    return SynchronousFuture(RouteLocation.uri(uri));
  }

  @override
  RouteInformation? restoreRouteInformation(RouteLocation configuration) {
    return RouteInformation(uri: configuration.uri);
  }
}
