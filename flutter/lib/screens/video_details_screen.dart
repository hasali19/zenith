import 'package:flutter/material.dart';
import 'package:zenith_flutter/screens/item_details.dart';

import '../api.dart' as api;

class VideoDetailsScreen extends StatefulWidget {
  final api.MediaItem item;

  const VideoDetailsScreen({Key? key, required this.item}) : super(key: key);

  @override
  State<VideoDetailsScreen> createState() => _VideoDetailsScreenState();
}

class _VideoDetailsScreenState extends State<VideoDetailsScreen> {
  @override
  Widget build(BuildContext context) {
    return ItemDetailsScreen(item: widget.item);
  }
}
