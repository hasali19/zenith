import 'dart:math';

import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';

class HeaderLayout extends MultiChildRenderObjectWidget {
  final double posterWidth;
  final double padding;
  final double separation;

  HeaderLayout({
    super.key,
    required Widget backdrop,
    required Widget poster,
    required Widget title,
    required Widget playButton,
    required Widget actions,
    required Widget body,
    this.posterWidth = 150,
    this.padding = 0,
    this.separation = 16,
  }) : super(children: [backdrop, poster, title, playButton, actions, body]);

  @override
  RenderObject createRenderObject(BuildContext context) {
    return _RenderHeaderLayout(
      posterWidth: posterWidth,
      padding: padding,
      separation: separation,
    );
  }

  @override
  void updateRenderObject(
      BuildContext context, covariant _RenderHeaderLayout renderObject) {
    renderObject
      ..posterWidth = posterWidth
      ..padding = padding
      ..separation = separation;
  }
}

class _HeaderChild extends ContainerBoxParentData<RenderBox>
    with ContainerParentDataMixin<RenderBox> {
  bool visible = true;
}

class _RenderHeaderLayout extends RenderBox
    with
        ContainerRenderObjectMixin<RenderBox, _HeaderChild>,
        RenderBoxContainerDefaultsMixin<RenderBox, _HeaderChild> {
  double posterWidth;
  double padding;
  double separation;

  _RenderHeaderLayout({
    required this.posterWidth,
    required this.padding,
    required this.separation,
  });

  @override
  void setupParentData(covariant RenderObject child) {
    child.parentData = _HeaderChild();
  }

  @override
  void performLayout() {
    final backdrop = firstChild!;
    final poster = childAfter(backdrop)!;
    final title = childAfter(poster)!;
    final playButton = childAfter(title)!;
    final actions = childAfter(playButton)!;
    final body = childAfter(actions)!;

    final backdropData = backdrop.parentData as _HeaderChild;
    final posterData = poster.parentData as _HeaderChild;
    final titleData = title.parentData as _HeaderChild;
    final playButtonData = playButton.parentData as _HeaderChild;
    final actionsData = actions.parentData as _HeaderChild;
    final bodyData = body.parentData as _HeaderChild;

    posterData.visible = true;
    titleData.visible = true;
    playButtonData.visible = true;
    actionsData.visible = true;
    bodyData.visible = true;

    if (constraints.maxWidth > 960) {
      backdropData.visible = false;

      backdrop.layout(BoxConstraints.tight(Size.zero));

      final lColConstraints =
          BoxConstraints(minWidth: posterWidth, maxWidth: posterWidth);

      poster.layout(lColConstraints, parentUsesSize: true);
      posterData.offset = Offset(padding, padding);

      final rColConstraints = BoxConstraints(
          maxWidth: constraints.maxWidth -
              posterData.offset.dx -
              posterWidth -
              separation -
              padding);

      title.layout(rColConstraints, parentUsesSize: true);
      titleData.offset =
          posterData.offset + Offset(posterWidth + separation, 0);

      playButton.layout(rColConstraints, parentUsesSize: true);
      playButtonData.offset =
          titleData.offset + Offset(0, title.size.height + 24);

      actions.layout(
          rColConstraints.copyWith(
              maxWidth: rColConstraints.maxWidth - playButton.size.width - 16),
          parentUsesSize: true);
      actionsData.offset = titleData.offset +
          Offset(playButton.size.width + 16, title.size.height + 24);

      body.layout(rColConstraints, parentUsesSize: true);
      bodyData.offset =
          playButtonData.offset + Offset(0, actions.size.height + 24);

      final lh = posterData.offset.dy + poster.size.height;
      final rh = bodyData.offset.dy + body.size.height;

      size = Size(constraints.maxWidth, max(lh, rh));
    } else {
      backdropData.visible = true;

      backdrop.layout(constraints.copyWith(minWidth: constraints.maxWidth),
          parentUsesSize: true);

      final lColConstraints =
          BoxConstraints(minWidth: posterWidth, maxWidth: posterWidth);

      poster.layout(lColConstraints, parentUsesSize: true);
      playButton.layout(lColConstraints, parentUsesSize: true);

      final rColConstraints = BoxConstraints(
          maxWidth: constraints.maxWidth -
              padding -
              posterWidth -
              separation -
              padding);

      title.layout(rColConstraints, parentUsesSize: true);
      actions.layout(rColConstraints, parentUsesSize: true);
      body.layout(BoxConstraints(maxWidth: constraints.maxWidth - padding * 2),
          parentUsesSize: true);

      final titleOffset =
          Offset(poster.size.width + padding + 16, backdrop.size.height + 8);

      final posterOffset = Offset(
          padding, titleOffset.dy - poster.size.height + title.size.height);
      final playButtonOffset = posterOffset + Offset(0, poster.size.height + 8);

      final actionsOffset = titleOffset +
          Offset(
              rColConstraints.maxWidth -
                  actions.getMinIntrinsicWidth(double.infinity),
              title.size.height + 8);
      final bodyOffset =
          playButtonOffset + Offset(0, playButton.size.height + 16);

      titleData.offset = titleOffset;
      actionsData.offset = actionsOffset;
      bodyData.offset = bodyOffset;

      posterData.offset = posterOffset;
      playButtonData.offset = playButtonOffset;

      size = Size(constraints.maxWidth, bodyOffset.dy + body.size.height);
    }
  }

  @override
  void paint(PaintingContext context, Offset offset) {
    var child = firstChild;
    while (child != null) {
      final childParentData = child.parentData! as _HeaderChild;
      if (childParentData.visible) {
        context.paintChild(child, childParentData.offset + offset);
      }
      child = childParentData.nextSibling;
    }
  }

  @override
  bool hitTestChildren(BoxHitTestResult result, {required Offset position}) {
    var child = lastChild;
    while (child != null) {
      // The x, y parameters have the top left of the node's box as the origin.
      final childParentData = child.parentData! as _HeaderChild;
      final bool isHit = childParentData.visible &&
          result.addWithPaintOffset(
            offset: childParentData.offset,
            position: position,
            hitTest: (BoxHitTestResult result, Offset transformed) {
              assert(transformed == position - childParentData.offset);
              return child!.hitTest(result, position: transformed);
            },
          );
      if (isHit) {
        return true;
      }
      child = childParentData.previousSibling;
    }
    return false;
  }
}
