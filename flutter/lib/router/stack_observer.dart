import 'package:flutter/material.dart';
import 'package:zenith/main.dart';
import 'package:zenith/router/stack_router.dart';

class StackObserver extends StatefulWidget {
  final void Function()? onPushNext;
  final void Function()? onPopNext;
  final Widget child;

  const StackObserver({
    super.key,
    this.onPushNext,
    this.onPopNext,
    required this.child,
  });

  @override
  State<StackObserver> createState() => _StackObserverState();
}

class _StackObserverState extends State<StackObserver> with RouteAware {
  StackRouterController? _controller;

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    _controller?.unsubscribe(this);
    _controller = StackRouter.anyOf(context);
    _controller?.subscribe(this);
  }

  @override
  void dispose() {
    super.dispose();
    _controller?.unsubscribe(this);
  }

  @override
  void didPushNext() {
    widget.onPushNext?.call();
  }

  @override
  void didPopNext() {
    widget.onPopNext?.call();
  }

  @override
  Widget build(BuildContext context) {
    return widget.child;
  }
}
