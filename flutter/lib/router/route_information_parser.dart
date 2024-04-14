import 'package:flutter/foundation.dart';
import 'package:flutter/widgets.dart';
import 'package:zenith/router/router_delegate.dart';

class ZenithRouteInformationParser extends RouteInformationParser<RouteConfig> {
  @override
  Future<RouteConfig> parseRouteInformation(RouteInformation routeInformation) {
    final uri = routeInformation.uri;
    var location = uri.path;
    if (uri.hasQuery) {
      location += '?${uri.query}';
    }
    return SynchronousFuture(RouteConfig(location));
  }

  @override
  RouteInformation? restoreRouteInformation(RouteConfig configuration) {
    return RouteInformation(uri: Uri.parse(configuration.location));
  }
}
