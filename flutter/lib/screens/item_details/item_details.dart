import 'dart:convert';
import 'dart:ui';

import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:sized_context/sized_context.dart';
import 'package:sliver_tools/sliver_tools.dart';
import 'package:zenith_flutter/api.dart';
import 'package:zenith_flutter/responsive.dart';
import 'package:zenith_flutter/router.dart';
import 'package:zenith_flutter/screens/item_details/episodes_list.dart';
import 'package:zenith_flutter/screens/item_details/header.dart';

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
  late Future<MediaItem> _item;
  ZenithApiClient get api => ref.watch(apiProvider);

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
    _refresh();
  }

  void _refresh() {
    setState(() {
      _item = api.fetchMediaItem(widget.id);
    });
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
      body: FutureBuilder<MediaItem>(
        future: _item,
        builder: (context, snapshot) {
          if (snapshot.hasError) {
            return const Center(child: Text("Failed to load data"));
          }
          if (!snapshot.hasData) {
            return const Center(child: CircularProgressIndicator());
          }
          final item = snapshot.data!;
          return Content(
            item: item,
            onRefresh: () async {
              _refresh();
              await _item;
            },
          );
        },
      ),
    );
  }
}

class Content extends ConsumerWidget {
  Content({
    Key? key,
    required this.item,
    required this.onRefresh,
  }) : super(key: key);

  final MediaItem item;
  final Future<void> Function() onRefresh;

  final _refresh = GlobalKey<RefreshIndicatorState>();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final api = ref.watch(apiProvider);
    final isDesktop = MediaQuery.of(context).isDesktop;
    final padding = isDesktop
        ? const EdgeInsets.fromLTRB(128, 128, 128, 32)
        : const EdgeInsets.fromLTRB(16, 96, 16, 16);

    void pushRoute(route) async {
      await context.router.push(route);
      _refresh.currentState?.show();
    }

    void onPlayPressed() async {
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
      onRefresh: onRefresh,
      triggerMode: RefreshIndicatorTriggerMode.anywhere,
      child: Stack(
        fit: StackFit.expand,
        children: [
          Backdrop(url: api.getMediaImageUrl(item.id, ImageType.backdrop)),
          BackdropBlur(
            child: CustomScrollView(
              slivers: [
                SliverCrossAxisPadded(
                  paddingStart: context.mq.padding.left,
                  paddingEnd: context.mq.padding.right,
                  child: MultiSliver(
                    children: [
                      SliverToBoxAdapter(
                        child: Padding(
                          padding: padding,
                          child: HeaderContent(
                            item: item,
                            onPlayPressed: onPlayPressed,
                          ),
                        ),
                      ),
                      if (item.type == MediaType.show)
                        EpisodesList(
                          id: item.id,
                          onEpisodePressed: onEpisodePressed,
                        ),
                      SliverToBoxAdapter(
                        child: SizedBox(height: context.mq.padding.bottom),
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
