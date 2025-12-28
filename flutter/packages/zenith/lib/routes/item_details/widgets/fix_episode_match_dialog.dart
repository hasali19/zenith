import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/api.dart';

class FixEpisodeMatchDialog extends ConsumerStatefulWidget {
  final MediaItem item;

  const FixEpisodeMatchDialog({super.key, required this.item});

  @override
  ConsumerState<FixEpisodeMatchDialog> createState() =>
      _FixEpisodeMatchDialogState();
}

class _FixEpisodeMatchDialogState extends ConsumerState<FixEpisodeMatchDialog> {
  late TextEditingController _season;
  late TextEditingController _episode;

  @override
  void initState() {
    super.initState();
    _season = TextEditingController(
      text: widget.item.grandparent!.index.toString(),
    );
    _episode = TextEditingController(
      text: widget.item.parent!.index.toString(),
    );
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Fix metadata match'),
      content: Row(
        // mainAxisSize: MainAxisSize.min,
        // crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Flexible(
            child: TextFormField(
              keyboardType: TextInputType.number,
              decoration: const InputDecoration(labelText: 'Season'),
              controller: _season,
            ),
          ),
          const SizedBox(width: 16),
          Flexible(
            child: TextFormField(
              keyboardType: TextInputType.number,
              decoration: const InputDecoration(labelText: 'Episode'),
              controller: _episode,
            ),
          ),
        ],
      ),
      actions: [
        TextButton(
          child: const Text('Cancel'),
          onPressed: () => Navigator.pop(context),
        ),
        TextButton(
          child: const Text('Ok'),
          onPressed: () async {
            final season = int.parse(_season.text);
            final episode = int.parse(_episode.text);

            await ref
                .read(apiProvider)
                .fixMetadataMatch(
                  widget.item.id,
                  FixMetadataMatch(season: season, episode: episode),
                );

            if (context.mounted) {
              Navigator.pop(context);
            }
          },
        ),
      ],
    );
  }
}
