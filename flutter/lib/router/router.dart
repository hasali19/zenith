import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:zenith/router/route_information_parser.dart';
import 'package:zenith/router/router_delegate.dart';

abstract class ZenithRouter {
  late final _routerDelegate = ZenithRouterDelegate(builder: build);
  final _routeInformationParser = ZenithRouteInformationParser();

  RouterDelegate<RouteLocation> get routerDelegate => _routerDelegate;
  RouteInformationParser<RouteLocation> get routeInformationParser =>
      _routeInformationParser;

  Widget build(BuildContext context);
}
