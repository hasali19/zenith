import 'dart:async';

import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:zenith/api.dart';
import 'package:zenith/image.dart';

part 'transcoder_page.g.dart';

@Riverpod(keepAlive: false)
Future<List<(TranscoderJob, MediaItem)>> _data(Ref ref) async {
  final api = ref.watch(apiProvider);

  final timer = Timer(Duration(seconds: 1), () => ref.invalidateSelf());

  ref.onDispose(timer.cancel);

  final transcoderState = await api.fetchTranscoderState();

  if (transcoderState.queue.isEmpty) {
    return [];
  }

  final items = await api
      .fetchMediaItems(transcoderState.queue.map((job) => job.itemId).toList());

  return transcoderState.queue.indexed.map((e) => (e.$2, items[e.$1])).toList();
}

@RoutePage()
class TranscoderPage extends ConsumerWidget {
  const TranscoderPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(_dataProvider);

    return Scaffold(
      appBar: AppBar(
        title: const Text('Transcoder'),
      ),
      body: switch (state) {
        AsyncData(value: final state) => CustomScrollView(
            slivers: [
              SliverList.builder(
                itemCount: state.length,
                itemBuilder: (context, index) {
                  final user = state[index];
                  return Column(
                    children: [
                      ListTile(
                        leading: Padding(
                          padding: const EdgeInsets.all(8.0),
                          child: ZenithApiImage(
                            id: user.$2.poster!,
                            requestWidth: 100,
                          ),
                        ),
                        title: Text(
                            user.$2.videoFile?.path.split('/').lastOrNull ??
                                'Unknown path'),
                        subtitle: switch (user.$1) {
                          Queued() => Text('Queued'),
                          Processing(:final progress) =>
                            LinearProgressIndicator(value: progress),
                        },
                      ),
                    ],
                  );
                },
              ),
            ],
          ),
        AsyncError(:final error) => Center(child: Text(error.toString())),
        _ => const Center(child: CircularProgressIndicator()),
      },
    );
  }
}
