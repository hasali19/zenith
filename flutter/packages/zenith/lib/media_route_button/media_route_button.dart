import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/media_route_button/media_route_button_controller.dart';
import 'package:zenith/media_route_chooser/media_route_chooser_dialog.dart';
import 'package:zenith/media_route_controller/media_route_controller_dialog.dart';

class MediaRouteButton extends ConsumerWidget {
  const MediaRouteButton({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(mediaRouteButtonControllerProvider);
    return IconButton(
      icon: Icon(_getIcon(state.isConnected)),
      onPressed: () => _onPressed(context, state.isConnected),
    );
  }

  IconData _getIcon(bool isConnected) {
    return isConnected ? Icons.cast_connected : Icons.cast;
  }

  void _onPressed(BuildContext context, bool isConnected) {
    if (isConnected) {
      showDialog(
        context: context,
        builder: (context) => const MediaRouteControllerDialog(),
      );
    } else {
      showDialog(
        context: context,
        builder: (context) => const MediaRouteChooserDialog(),
      );
    }
  }
}
