import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/api.dart';

class DeleteConfirmationDialog extends ConsumerStatefulWidget {
  final int id;

  const DeleteConfirmationDialog({super.key, required this.id});

  @override
  ConsumerState<ConsumerStatefulWidget> createState() =>
      _DeleteConfirmationDialogState();
}

class _DeleteConfirmationDialogState
    extends ConsumerState<DeleteConfirmationDialog> {
  bool _removeFiles = true;
  bool _isInProgress = false;

  @override
  Widget build(BuildContext context) {
    return PopScope(
      canPop: !_isInProgress,
      child: AlertDialog(
        title: const Text('Delete item'),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            const Text(
                'Are you sure you want to permanently delete this item?'),
            const SizedBox(height: 16),
            Row(
              children: [
                Checkbox(
                  value: _removeFiles,
                  onChanged: _isInProgress ? null : _onRemoveFilesToggled,
                ),
                const SizedBox(width: 12),
                const Text('Remove files'),
              ],
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: _isInProgress ? null : () => Navigator.pop(context),
            child: const Text('Cancel'),
          ),
          ElevatedButton(
            onPressed: _isInProgress ? null : () => _onDeleteConfirmed(context),
            child: const Text('Delete'),
          ),
        ],
      ),
    );
  }

  void _onRemoveFilesToggled(bool? value) {
    setState(() => _removeFiles = value!);
  }

  Future<void> _onDeleteConfirmed(BuildContext context) async {
    setState(() => _isInProgress = true);

    try {
      await ref
          .read(apiProvider)
          .deleteMediaItem(widget.id, removeFiles: _removeFiles);
    } catch (e) {
      if (context.mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text('Failed to delete media item')));
      }
    } finally {
      setState(() => _isInProgress = false);
    }

    if (context.mounted) {
      Navigator.pop(context, true);
    }
  }
}
