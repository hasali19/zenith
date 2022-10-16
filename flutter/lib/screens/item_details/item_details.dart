import 'dart:convert';
import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:zenith_flutter/api.dart';
import 'package:zenith_flutter/responsive.dart';
import 'package:zenith_flutter/screens/item_details/episodes_list.dart';
import 'package:zenith_flutter/screens/item_details/header.dart';

final transparentImage = base64Decode(
    "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=");

class ItemDetailsScreen extends StatefulWidget {
  final int id;

  const ItemDetailsScreen({Key? key, required this.id}) : super(key: key);

  @override
  State<ItemDetailsScreen> createState() => _ItemDetailsScreenState();
}

class _ItemDetailsScreenState extends State<ItemDetailsScreen> {
  late Future<MediaItem> _item;

  @override
  void initState() {
    super.initState();
    _item = fetchMediaItem(widget.id);
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
          return Content(item: item);
        },
      ),
    );
  }
}

class Content extends StatelessWidget {
  const Content({Key? key, required this.item}) : super(key: key);

  final MediaItem item;

  @override
  Widget build(BuildContext context) {
    final isDesktop = MediaQuery.of(context).isDesktop;
    final padding = isDesktop
        ? const EdgeInsets.fromLTRB(128, 128, 128, 32)
        : const EdgeInsets.fromLTRB(16, 96, 16, 16);
    return Stack(
      fit: StackFit.expand,
      children: [
        Backdrop(url: getMediaImageUrl(item.id, ImageType.backdrop)),
        BackdropBlur(
          child: CustomScrollView(
            slivers: [
              SliverToBoxAdapter(
                child: Padding(
                  padding: padding,
                  child: HeaderContent(item: item),
                ),
              ),
              if (item.type == MediaType.show) EpisodesList(id: item.id),
            ],
          ),
        ),
      ],
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
