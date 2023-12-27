import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/media_route_controller/media_route_controller_cubit.dart';
import 'package:zenith/remote_playback.dart';

class MediaRouteControllerDialog extends ConsumerWidget {
  const MediaRouteControllerDialog({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return BlocProvider(
      create: (context) =>
          MediaRouteControllerCubit(context.read<MediaRouter>()),
      child: BlocConsumer<MediaRouteControllerCubit, MediaRouteControllerState>(
        listenWhen: (previous, current) => previous.route != null,
        listener: (context, state) {
          if (state.route == null) {
            Navigator.pop(context);
          }
        },
        builder: (context, state) => AlertDialog(
          title: Text(state.route?.name ?? ''),
          actions: [
            TextButton(
              onPressed: () =>
                  context.read<MediaRouteControllerCubit>().deselectRoute(),
              child: const Text('Disconnect'),
            ),
          ],
        ),
      ),
    );
  }
}
