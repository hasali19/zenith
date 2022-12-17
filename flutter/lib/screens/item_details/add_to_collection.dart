import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/api.dart';
import 'package:zenith/screens/collections.dart';

class AddToCollectionDialog extends ConsumerStatefulWidget {
  final int id;

  const AddToCollectionDialog({
    Key? key,
    required this.id,
  }) : super(key: key);

  @override
  ConsumerState<AddToCollectionDialog> createState() =>
      _AddToCollectionDialogState();
}

class _AddToCollectionDialogState extends ConsumerState<AddToCollectionDialog> {
  Collection? _selected;
  late TextEditingController _controller;

  @override
  void initState() {
    super.initState();
    _controller = TextEditingController();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted) {
        ref.invalidate(collectionsProvider);
      }
    });
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final collections = ref.watch(collectionsProvider);
    return AlertDialog(
      title: const Text("Add to collection"),
      content: Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          DropdownButton<Collection>(
            isExpanded: true,
            value: _selected,
            items: [
              DropdownMenuItem(
                value: null,
                child: Row(
                  children: const [
                    Icon(Icons.add),
                    SizedBox(width: 8),
                    Text("Create new"),
                  ],
                ),
              ),
              if (collections.hasValue)
                ...collections.requireValue
                    .map((e) => DropdownMenuItem(
                          value: e,
                          child: Text(e.name),
                        ))
                    .toList(),
            ],
            onChanged: (value) {
              setState(() => _selected = value);
            },
          ),
          if (_selected == null)
            TextField(
              controller: _controller,
              decoration: const InputDecoration(
                hintText: "Name",
              ),
            ),
        ],
      ),
      actions: [
        TextButton(
          child: const Text("Cancel"),
          onPressed: () => Navigator.pop(context),
        ),
        TextButton(
          child: const Text("Ok"),
          onPressed: () async {
            var selected = _selected;
            if (selected == null) {
              final name = _controller.text;
              selected = await ref.read(apiProvider).createCollection(name);
            }

            final items =
                await ref.read(apiProvider).fetchCollectionItems(selected.id);

            await ref.read(apiProvider).updateCollection(
                selected.id, [...items.map((e) => e.id), widget.id]);

            Navigator.pop(context);
          },
        ),
      ],
    );
  }
}
