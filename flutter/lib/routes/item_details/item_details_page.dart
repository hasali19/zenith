import 'dart:ui';

import 'package:auto_route/auto_route.dart';
import 'package:cast_framework/cast_framework.dart' show CastFrameworkPlatform;
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:gap/gap.dart';
import 'package:sized_context/sized_context.dart';
import 'package:sliver_tools/sliver_tools.dart';
import 'package:zenith/api.dart';
import 'package:zenith/fade_in_image.dart';
import 'package:zenith/language_codes.dart';
import 'package:zenith/media_route_button/media_route_button.dart';
import 'package:zenith/responsive.dart';
import 'package:zenith/router.dart';
import 'package:zenith/routes/item_details/item_details_controller.dart';
import 'package:zenith/routes/item_details/item_details_state.dart';
import 'package:zenith/routes/item_details/widgets/delete_confirmation_dialog.dart';
import 'package:zenith/routes/item_details/widgets/episodes_list.dart';
import 'package:zenith/routes/item_details/widgets/fix_episode_match_dialog.dart';
import 'package:zenith/routes/item_details/widgets/header.dart';
import 'package:zenith/theme.dart';

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
                ref.refresh(itemDetailsControllerProvider(id).future),
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
          if (isDesktop)
            ZenithFadeInImage.dio(
              url: widget.state.backdropUrl,
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
                      SliverToBoxAdapter(
                        child: Padding(
                          padding: EdgeInsets.symmetric(
                            horizontal: isDesktop ? 128 : 16,
                            vertical: 32,
                          ),
                          child: Column(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              Text(
                                'Video Info',
                                style: context.zenithTheme.titleMedium,
                              ),
                              const Gap(8),
                              DefaultTextStyle(
                                style: context.zenithTheme.bodyMedium,
                                child: Table(
                                  defaultColumnWidth:
                                      const IntrinsicColumnWidth(),
                                  columnWidths: {
                                    1: const IntrinsicColumnWidth(),
                                    2: isDesktop
                                        ? const IntrinsicColumnWidth()
                                        : const FlexColumnWidth(),
                                  },
                                  children: [
                                    TableRow(
                                      children: [
                                        Text('Format'),
                                        const SizedBox(width: 16),
                                        Text(widget
                                            .state.item.videoFile!.format),
                                      ],
                                    ),
                                    TableRow(
                                      children: [
                                        Text('Path'),
                                        const SizedBox(width: 16),
                                        Text(widget.state.item.videoFile!.path),
                                      ],
                                    ),
                                  ],
                                ),
                              ),
                              for (final stream
                                  in widget.state.item.videoFile!.streams) ...[
                                const Gap(8),
                                Text(
                                  'Stream #${stream.index}',
                                  style: const TextStyle(
                                      fontWeight: FontWeight.bold),
                                ),
                                DefaultTextStyle(
                                  style: context.zenithTheme.bodyMedium,
                                  child: Table(
                                    defaultColumnWidth:
                                        const IntrinsicColumnWidth(),
                                    columnWidths: {
                                      1: const IntrinsicColumnWidth(),
                                      2: isDesktop
                                          ? const IntrinsicColumnWidth()
                                          : const FlexColumnWidth(),
                                    },
                                    children: [
                                      TableRow(
                                        children: [
                                          const Text('Type'),
                                          const SizedBox(width: 16),
                                          Text(switch (stream) {
                                            AudioStreamInfo() => 'Audio',
                                            VideoStreamInfo() => 'Video',
                                            _ => 'Unknown',
                                          }),
                                        ],
                                      ),
                                      TableRow(
                                        children: [
                                          const Text('Codec'),
                                          const SizedBox(width: 16),
                                          Text(stream.codec),
                                        ],
                                      ),
                                      if (stream is AudioStreamInfo)
                                        TableRow(
                                          children: [
                                            const Text('Language'),
                                            const SizedBox(width: 16),
                                            Text(tryResolveLanguageCode(
                                                stream.language ?? 'Unkown')),
                                          ],
                                        ),
                                      if (stream is VideoStreamInfo)
                                        TableRow(
                                          children: [
                                            Text('Size'),
                                            const SizedBox(width: 16),
                                            Text(
                                                '${stream.width}x${stream.height}'),
                                          ],
                                        ),
                                      if (stream is VideoStreamInfo)
                                        TableRow(
                                          children: [
                                            Text('Crop Size'),
                                            const SizedBox(width: 16),
                                            Text(
                                                '(x1: ${stream.cropX1}, y1: ${stream.cropY1}) (x2: ${stream.cropX2}, y2: ${stream.cropY2})'),
                                          ],
                                        ),
                                    ],
                                  ),
                                ),
                              ],
                            ],
                          ),
                        ),
                      ),
                      if (widget.state.seasons.isNotEmpty)
                        EpisodesList(
                          groups: widget.state.seasons,
                          onEpisodePressed: onEpisodePressed,
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
      context.router.pop();
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
