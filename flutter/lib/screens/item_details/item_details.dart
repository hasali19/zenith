import 'dart:convert';
import 'dart:ui';

import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:sized_context/sized_context.dart';
import 'package:sliver_tools/sliver_tools.dart';
import 'package:zenith/api.dart';
import 'package:zenith/responsive.dart';
import 'package:zenith/router.dart';
import 'package:zenith/screens/item_details/episodes_list.dart';
import 'package:zenith/screens/item_details/header.dart';
import 'package:zenith/screens/item_details/model.dart';

final transparentImage = base64Decode(
    "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=");

class ItemDetailsScreen extends ConsumerStatefulWidget {
  final int id;

  const ItemDetailsScreen({
    Key? key,
    @pathParam required this.id,
  }) : super(key: key);

  @override
  ConsumerState<ItemDetailsScreen> createState() => _ItemDetailsScreenState();
}

class _ItemDetailsScreenState extends ConsumerState<ItemDetailsScreen> {
  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    ref.invalidate(itemDetailsModelProvider(widget.id));
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      extendBodyBehindAppBar: true,
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        elevation: 0,
        scrolledUnderElevation: 0,
      ),
      body: Consumer(builder: (context, ref, child) {
        final model = ref.watch(itemDetailsModelProvider(widget.id));
        return model.when(
          loading: () => const Center(child: CircularProgressIndicator()),
          error: (error, stackTrace) => Center(child: Text("$error")),
          data: (data) => Content(
              model: data,
              onRefresh: () =>
                  ref.refresh(itemDetailsModelProvider(widget.id).future)),
        );
      }),
    );
  }
}

class Content extends ConsumerStatefulWidget {
  const Content({
    Key? key,
    required this.model,
    required this.onRefresh,
  }) : super(key: key);

  final ItemDetailsModel model;
  final Future<void> Function() onRefresh;

  @override
  ConsumerState<ConsumerStatefulWidget> createState() => _ContentState();
}

class _ContentState extends ConsumerState<Content> {
  final _refresh = GlobalKey<RefreshIndicatorState>();

  @override
  Widget build(BuildContext context) {
    final api = ref.watch(apiProvider);
    final isDesktop = MediaQuery.of(context).isDesktop;

    void pushRoute(route) async {
      await context.router.push(route);
      _refresh.currentState?.show();
    }

    void onPlayPressed(MediaItem item) async {
      pushRoute(VideoPlayerScreenRoute(
        id: item.id,
        startPosition:
            item.shouldResume ? item.videoUserData?.position ?? 0 : 0,
      ));
    }

    void onEpisodePressed(MediaItem episode) async {
      pushRoute(ItemDetailsScreenRoute(id: episode.id));
    }

    return RefreshIndicator(
      key: _refresh,
      onRefresh: widget.onRefresh,
      triggerMode: RefreshIndicatorTriggerMode.anywhere,
      child: Stack(
        fit: StackFit.expand,
        children: [
          if (isDesktop)
            Backdrop(
              url: api.getMediaImageUrl(
                  widget.model.item.id, ImageType.backdrop),
            ),
          BackdropBlur(
            child: CustomScrollView(
              slivers: [
                SliverCrossAxisPadded(
                  paddingStart: context.mq.padding.left,
                  paddingEnd: context.mq.padding.right,
                  child: MultiSliver(
                    children: [
                      SliverToBoxAdapter(
                        child: HeaderContent(
                          model: widget.model,
                          refresh: () => _refresh.currentState?.show(),
                          onPlayPressed: onPlayPressed,
                          onViewItemDetails: (id) =>
                              pushRoute(ItemDetailsScreenRoute(id: id)),
                        ),
                      ),
                      if (widget.model.item.type == MediaType.show)
                        EpisodesList(
                          model: widget.model,
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
}

class Backdrop extends StatelessWidget {
  final String url;

  const Backdrop({
    Key? key,
    required this.url,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return FadeInImage.memoryNetwork(
      placeholder: transparentImage,
      image: url,
      fit: BoxFit.cover,
    );
  }
}

class BackdropBlur extends StatelessWidget {
  const BackdropBlur({Key? key, required this.child}) : super(key: key);

  final Widget child;

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
