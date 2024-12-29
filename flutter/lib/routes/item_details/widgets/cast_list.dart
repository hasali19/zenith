import 'package:flutter/material.dart';
import 'package:zenith/api.dart';
import 'package:zenith/constants.dart';
import 'package:zenith/image.dart';
import 'package:zenith/responsive.dart';

class CastList extends StatelessWidget {
  final List<CastMember> cast;

  const CastList({super.key, required this.cast});

  @override
  Widget build(BuildContext context) {
    final isDesktop = context.isDesktop;

    final double itemHeight = isDesktop ? 240 : 165;
    final double itemWidth = itemHeight / 3 * 2;
    final double itemSpacing = isDesktop ? 8 : 4;
    final double nameSize = isDesktop ? 16 : 13;
    final double horizontalPadding = isDesktop ? 128 : 16;
    final double topPadding = isDesktop ? 48 : 16;

    return SizedBox(
      height: itemHeight + topPadding,
      child: ListView.builder(
        padding: EdgeInsets.only(
          left: horizontalPadding - itemSpacing,
          right: horizontalPadding - itemSpacing,
          top: topPadding,
        ),
        scrollDirection: Axis.horizontal,
        itemCount: cast.length,
        itemBuilder: (context, index) {
          final castMember = cast[index];
          return _CastListItem(
            itemSpacing: itemSpacing,
            itemWidth: itemWidth,
            castMember: castMember,
            itemHeight: itemHeight,
            nameSize: nameSize,
          );
        },
      ),
    );
  }
}

class _CastListItem extends StatelessWidget {
  const _CastListItem({
    required this.itemSpacing,
    required this.itemWidth,
    required this.castMember,
    required this.itemHeight,
    required this.nameSize,
  });

  final double itemSpacing;
  final double itemWidth;
  final CastMember castMember;
  final double itemHeight;
  final double nameSize;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: EdgeInsets.symmetric(horizontal: itemSpacing),
      child: SizedBox(
        width: itemWidth,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Card(
              margin: EdgeInsets.zero,
              clipBehavior: Clip.antiAlias,
              child: Stack(
                children: [
                  _buildProfileImage(),
                  Positioned(
                    left: 8,
                    right: 8,
                    bottom: 8,
                    child: _buildTextContent(),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildProfileImage() {
    return switch (castMember.profile) {
      null => SizedBox(
          width: itemWidth,
          height: itemHeight,
          child: const Center(
            child: Icon(Icons.person, size: 48),
          ),
        ),
      final profile => ZenithApiImage(
          id: profile,
          requestWidth: mediaProfileImageWidth,
          width: itemWidth,
          height: itemHeight,
          fit: BoxFit.cover,
        ),
    };
  }

  Column _buildTextContent() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Text(
          castMember.name,
          maxLines: 2,
          overflow: TextOverflow.ellipsis,
          style: TextStyle(
            fontSize: nameSize,
            color: Colors.white,
            shadows: const [Shadow(color: Colors.black, blurRadius: 4)],
          ),
        ),
        if (castMember.character != null && castMember.character!.isNotEmpty)
          Text(
            castMember.character!,
            maxLines: 1,
            overflow: TextOverflow.ellipsis,
            style: TextStyle(
              fontSize: nameSize - 2,
              color: Colors.white,
              shadows: const [Shadow(color: Colors.black, blurRadius: 4)],
            ),
          ),
      ],
    );
  }
}
