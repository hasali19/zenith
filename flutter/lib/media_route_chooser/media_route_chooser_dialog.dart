import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:zenith/media_route_chooser/media_route_chooser_cubit.dart';
import 'package:zenith/remote_playback.dart';

class MediaRouteChooserDialog extends StatelessWidget {
  const MediaRouteChooserDialog({super.key});

  @override
  Widget build(BuildContext context) {
    return BlocProvider(
      create: (context) => MediaRouteChooserCubit(context.read<MediaRouter>()),
      child: BlocConsumer<MediaRouteChooserCubit, MediaRouteChooserState>(
        listenWhen: (previous, current) => !previous.isConnected,
        listener: (context, state) {
          if (state.isConnected) {
            Navigator.pop(context);
          }
        },
        builder: (context, state) => AlertDialog(
          title: const Text('Cast'),
          content: SizedBox(
            width: double.maxFinite,
            child: _buildRouteList(
                state,
                context,
                (route) =>
                    context.read<MediaRouteChooserCubit>().selectRoute(route)),
          ),
        ),
      ),
    );
  }

  Widget _buildRouteList(MediaRouteChooserState state, BuildContext context,
      void Function(MediaRoute) onTap) {
    final routeListItems = state.routes
        .map(
          (route) => _buildRouteListItem(
            route,
            isRequested: state.requestedId == route.id,
            onTap: () => onTap(route),
          ),
        )
        .toList();

    return ListView(
      shrinkWrap: true,
      children: routeListItems,
    );
  }

  Widget _buildRouteListItem(
    MediaRoute route, {
    required bool isRequested,
    required void Function() onTap,
  }) {
    return ListTile(
      title: Text(route.name),
      subtitle: route.description == null ? null : Text(route.description!),
      leading: const Icon(Icons.tv),
      trailing: isRequested ? const CircularProgressIndicator() : null,
      onTap: onTap,
    );
  }
}
