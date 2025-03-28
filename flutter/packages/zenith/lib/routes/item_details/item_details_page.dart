import 'dart:convert';
import 'dart:ui';

import 'package:auto_route/auto_route.dart';
import 'package:cast_framework/cast_framework.dart' show CastFrameworkPlatform;
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:sized_context/sized_context.dart';
import 'package:sliver_tools/sliver_tools.dart';
import 'package:zenith/api.dart';
import 'package:zenith/constants.dart';
import 'package:zenith/image.dart';
import 'package:zenith/media_route_button/media_route_button.dart';
import 'package:zenith/responsive.dart';
import 'package:zenith/router.dart';
import 'package:zenith/routes/item_details/item_details_controller.dart';
import 'package:zenith/routes/item_details/item_details_state.dart';
import 'package:zenith/routes/item_details/widgets/delete_confirmation_dialog.dart';
import 'package:zenith/routes/item_details/widgets/episodes_list.dart';
import 'package:zenith/routes/item_details/widgets/fix_episode_match_dialog.dart';
import 'package:zenith/routes/item_details/widgets/header.dart';

import 'widgets/cast_list.dart';

@RoutePage()
class ItemDetailsPage extends ConsumerWidget {
  final int id;

  const ItemDetailsPage({
    super.key,
    @pathParam required this.id,
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Scaffold(
      extendBodyBehindAppBar: true,
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        foregroundColor: Colors.white,
        elevation: 0,
        scrolledUnderElevation: 0,
        actions: [
          if (CastFrameworkPlatform.instance.isSupported)
            const MediaRouteButton()
        ],
      ),
      body: Consumer(builder: (context, ref, child) {
        final model = ref.watch(itemDetailsControllerProvider(id));
        return model.when(
          loading: () => const Center(child: CircularProgressIndicator()),
          error: (error, stackTrace) => Center(child: Text('$error')),
          data: (data) => _ItemDetailsContent(
            state: data,
            onRefresh: () =>
                ref.read(itemDetailsControllerProvider(id).notifier).refresh(),
          ),
        );
      }),
    );
  }
}

class _ItemDetailsContent extends ConsumerStatefulWidget {
  const _ItemDetailsContent({
    required this.state,
    required this.onRefresh,
  });

  final ItemDetailsState state;
  final Future<void> Function() onRefresh;

  @override
  ConsumerState<ConsumerStatefulWidget> createState() =>
      _ItemDetailsContentState();
}

class _ItemDetailsContentState extends ConsumerState<_ItemDetailsContent> {
  final _refresh = GlobalKey<RefreshIndicatorState>();

  @override
  void initState() {
    super.initState();

    Future.microtask(
      () async {
        final api = ref.read(apiProvider);
        final token = await api.getAccessToken(AccessTokenOwner.system, 'cast',
            create: true);
        if (CastFrameworkPlatform.instance.isSupported) {
          CastFrameworkPlatform.instance.remoteMediaClient.sendMessage(
            'urn:x-cast:dev.hasali.zenith',
            jsonEncode({
              'type': 'init',
              'token': token.token,
              'server': api.baseUrl,
            }),
          );

          CastFrameworkPlatform.instance.remoteMediaClient.sendMessage(
            'urn:x-cast:dev.hasali.zenith',
            jsonEncode({
              'type': 'focus-item-details',
              'id': widget.state.item.id,
            }),
          );
        }
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    final isDesktop = MediaQuery.of(context).isDesktop;

    void pushRoute(route) async {
      await context.router.push(route);
      _refresh.currentState?.show();
    }

    void onPlayPressed() async {
      final item = widget.state.playable;
      if (item == null) {
        return;
      }

      pushRoute(VideoPlayerRoute(
        id: item.id,
        startPosition: item.playPosition,
      ));
    }

    void onEpisodePressed(EpisodeState episode) async {
      pushRoute(ItemDetailsRoute(id: episode.id));
    }

    return RefreshIndicator(
      key: _refresh,
      onRefresh: widget.onRefresh,
      triggerMode: RefreshIndicatorTriggerMode.anywhere,
      child: Stack(
        fit: StackFit.expand,
        children: [
          if (isDesktop && widget.state.backdrop != null)
            ZenithApiImage(
              id: widget.state.backdrop!,
              requestWidth: mediaBackdropImageWidth,
            ),
          _BackdropBlur(
            child: CustomScrollView(
              slivers: [
                SliverCrossAxisPadded(
                  paddingStart: context.mq.padding.left,
                  paddingEnd: context.mq.padding.right,
                  child: MultiSliver(
                    children: [
                      SliverToBoxAdapter(
                        child: HeaderContent(
                          state: widget.state,
                          onPlayPressed: onPlayPressed,
                          onChildItemPressed: (id) =>
                              pushRoute(ItemDetailsRoute(id: id)),
                          onFindMetadataMatch: _onFindMetadataMatch,
                          onFixEpisodeMatch: _onFixEpisodeMatch,
                          onRefreshMetadata: _onRefreshMetadata,
                          onDelete: _onDelete,
                        ),
                      ),
                      if (widget.state.item.cast.isNotEmpty)
                        CastList(cast: widget.state.item.cast),
                      if (widget.state.seasons.isNotEmpty)
                        SliverPadding(
                          padding: EdgeInsets.only(top: 16),
                          sliver: EpisodesList(
                            initialExpanded: widget.state.playable?.seasonIndex,
                            groups: widget.state.seasons,
                            onEpisodePressed: onEpisodePressed,
                          ),
                        ),
                      SliverToBoxAdapter(
                        child: SizedBox(height: context.mq.padding.bottom + 16),
                      ),
                    ],
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  void _onFindMetadataMatch() {
    final id = widget.state.item.id;
    final provider = itemDetailsControllerProvider(id);
    ref.read(provider.notifier).findMetadataMatch();
  }

  void _onFixEpisodeMatch() async {
    await showDialog(
      context: context,
      builder: (context) => FixEpisodeMatchDialog(
        item: widget.state.item,
      ),
    );
    _refresh.currentState?.show();
  }

  void _onRefreshMetadata() {
    final id = widget.state.item.id;
    final provider = itemDetailsControllerProvider(id);
    ref.read(provider.notifier).refreshMetadata();
  }

  void _onDelete() async {
    final context = this.context;

    final bool? result = await showDialog(
      context: context,
      builder: (context) => DeleteConfirmationDialog(id: widget.state.item.id),
    );

    if (result == true && context.mounted) {
      context.router.maybePop();
    }
  }
}

class _BackdropBlur extends StatelessWidget {
  final Widget child;

  const _BackdropBlur({required this.child});

  @override
  Widget build(BuildContext context) {
    final isDesktop = context.isDesktop;
    if (!isDesktop) return child;

    final isDarkTheme = Theme.of(context).brightness == Brightness.dark;
    final color = isDarkTheme ? Colors.black : Colors.white;
    return BackdropFilter(
      filter: ImageFilter.blur(sigmaX: 10, sigmaY: 10),
      child: Container(
        decoration: BoxDecoration(
          color: color.withOpacity(0.5),
        ),
        child: child,
      ),
    );
  }
}
