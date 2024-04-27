import 'package:flutter/material.dart';
import 'package:zenith/router/route_information_parser.dart';
import 'package:zenith/router/router_delegate.dart';

abstract class ZenithRouter {
  RouterConfig<RouteConfig> config() {
    return RouterConfig(
      routerDelegate: ZenithRouterDelegate(builder: build),
      routeInformationParser: ZenithRouteInformationParser(),
    );
  }

  Widget build(BuildContext context);
}
