import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:zenith/media_route_button/media_route_button_cubit.dart';
import 'package:zenith/media_route_chooser/media_route_chooser_dialog.dart';
import 'package:zenith/media_route_controller/media_route_controller_dialog.dart';
import 'package:zenith/remote_playback.dart';

class MediaRouteButton extends StatelessWidget {
  const MediaRouteButton({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocProvider(
      create: (context) => MediaRouteButtonCubit(context.read<MediaRouter>()),
      child: BlocBuilder<MediaRouteButtonCubit, MediaRouteButtonState>(
        builder: (context, state) => IconButton(
          icon: Icon(_getIcon(state.isConnected)),
          onPressed: () => _onPressed(context, state.isConnected),
        ),
      ),
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
