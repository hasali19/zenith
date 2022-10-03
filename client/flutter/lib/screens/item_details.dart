import 'dart:convert';
import 'dart:ui';

import 'package:flutter/material.dart';
import 'package:zenith_flutter/responsive.dart';

final transparentImage = base64Decode(
    "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNkYAAAAAYAAjCB0C8AAAAASUVORK5CYII=");

class ItemDetailsScreen extends StatelessWidget {
  final String poster;
  final String backdrop;
  final String name;
  final int? year;
  final String? overview;
  final Widget? body;

  const ItemDetailsScreen({
    Key? key,
    required this.poster,
    required this.backdrop,
    required this.name,
    this.year,
    this.overview,
    this.body,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final isDark = Theme.of(context).brightness == Brightness.dark;
    final isDesktop = MediaQuery.of(context).isDesktop;
    return Scaffold(
      extendBodyBehindAppBar: true,
      appBar: AppBar(
        backgroundColor: Colors.transparent,
        elevation: 0,
        scrolledUnderElevation: 0,
      ),
      body: Stack(
        fit: StackFit.expand,
        children: [
          Backdrop(url: backdrop),
          BackdropFilter(
            filter: ImageFilter.blur(sigmaX: 10, sigmaY: 10),
            child: Container(
              decoration: BoxDecoration(
                  color:
                      (isDark ? Colors.black : Colors.white).withOpacity(0.5)),
              child: CustomScrollView(
                slivers: [
                  SliverToBoxAdapter(
                    child: Padding(
                      padding: isDesktop
                          ? const EdgeInsets.fromLTRB(128, 128, 128, 32)
                          : const EdgeInsets.fromLTRB(16, 96, 16, 16),
                      child: ItemDetails(
                        poster: poster,
                        name: name,
                        year: year,
                        overview: overview,
                      ),
                    ),
                  ),
                  if (body != null) body!,
                ],
              ),
            ),
          )
        ],
      ),
    );
  }
}

class ItemDetails extends StatelessWidget {
  const ItemDetails({
    Key? key,
    required this.poster,
    required this.name,
    this.year,
    this.overview,
  }) : super(key: key);

  final String poster;
  final String name;
  final int? year;
  final String? overview;

  @override
  Widget build(BuildContext context) {
    final isDesktop = MediaQuery.of(context).isDesktop;
    if (isDesktop) {
      return Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SizedBox(width: 300, child: Poster(url: poster)),
          const SizedBox(width: 48),
          Flexible(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Title(text: name),
                if (year != null)
                  Padding(
                    padding: const EdgeInsets.only(top: 16.0),
                    child: Subtitle(text: "$year"),
                  ),
                const SizedBox(height: 32),
                if (overview != null)
                  Container(
                    constraints: const BoxConstraints(maxWidth: 600),
                    child: Overview(text: overview!),
                  ),
              ],
            ),
          ),
        ],
      );
    } else {
      return Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Flexible(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Center(
                  child: SizedBox(width: 200, child: Poster(url: poster)),
                ),
                const SizedBox(height: 48),
                Title(text: name),
                if (year != null)
                  Padding(
                    padding: const EdgeInsets.only(top: 16.0),
                    child: Subtitle(text: "$year"),
                  ),
                const SizedBox(height: 32),
                if (overview != null) Overview(text: overview!),
              ],
            ),
          ),
        ],
      );
    }
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

class Poster extends StatelessWidget {
  final String url;

  const Poster({
    Key? key,
    required this.url,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return ClipRRect(
      borderRadius: BorderRadius.circular(16),
      child: AspectRatio(
        aspectRatio: 2.0 / 3.0,
        child: Image.network(url),
      ),
    );
  }
}

class Title extends StatelessWidget {
  final String text;

  const Title({
    Key? key,
    required this.text,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final style = theme.textTheme.headline3;
    return Text(text, style: style);
  }
}

class Subtitle extends StatelessWidget {
  final String text;

  const Subtitle({
    Key? key,
    required this.text,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final style = theme.textTheme.headline5;
    return Text(text, style: style);
  }
}

class Overview extends StatelessWidget {
  final String text;

  const Overview({
    Key? key,
    required this.text,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final style = theme.textTheme.bodyLarge!.copyWith(fontSize: 16);
    return Text(text, style: style);
  }
}
