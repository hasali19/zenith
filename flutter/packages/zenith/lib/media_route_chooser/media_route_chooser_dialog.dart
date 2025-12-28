import 'package:cast_framework/cast_framework.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/media_route_chooser/media_route_chooser_controller.dart';

class MediaRouteChooserDialog extends ConsumerWidget {
  const MediaRouteChooserDialog({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    ref.listen(mediaRouteChooserControllerProvider, (previous, next) {
      if ((previous == null || !previous.isConnected) && next.isConnected) {
        Navigator.pop(context);
      }
    });

    final state = ref.watch(mediaRouteChooserControllerProvider);
    return AlertDialog(
      title: const Text('Cast'),
      content: SizedBox(
        width: double.maxFinite,
        child: _buildRouteList(
          state,
          context,
          (route) => ref
              .read(mediaRouteChooserControllerProvider.notifier)
              .selectRoute(route),
        ),
      ),
    );
  }

  Widget _buildRouteList(
    MediaRouteChooserState state,
    BuildContext context,
    void Function(MediaRoute) onTap,
  ) {
    final routeListItems = state.routes
        .map(
          (route) => _buildRouteListItem(
            route,
            isRequested: state.requestedId == route.id,
            onTap: () => onTap(route),
          ),
        )
        .toList();

    return ListView(shrinkWrap: true, children: routeListItems);
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
