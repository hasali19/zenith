import 'package:flutter/material.dart';
import 'package:zenith/updater.dart';

class UpdateDialog extends StatefulWidget {
  const UpdateDialog({
    super.key,
    required this.update,
  });

  final Update update;

  @override
  State<UpdateDialog> createState() => _UpdateDialogState();
}

class _UpdateDialogState extends State<UpdateDialog> {
  bool _isUpdating = false;
  double _progress = 0;

  @override
  Widget build(BuildContext context) {
    final Widget content;
    if (_isUpdating) {
      content = Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            children: [
              const Expanded(child: Text('Downloading...')),
              Text('${_progress.toStringAsFixed(2)}M'),
            ],
          ),
          const SizedBox(height: 16),
          const LinearProgressIndicator(),
        ],
      );
    } else {
      content = const Text('An update is available');
    }

    return AlertDialog(
      title: const Text('Update'),
      content: content,
      actions: [
        TextButton(
          onPressed: !_isUpdating ? () => Navigator.pop(context) : null,
          child: const Text('Cancel'),
        ),
        TextButton(
          onPressed: !_isUpdating ? _onApplyUpdate : null,
          child: const Text('Install'),
        ),
      ],
    );
  }

  void _onApplyUpdate() async {
    setState(() {
      _isUpdating = true;
    });

    await widget.update.install(
      (progress) => setState(() {
        _progress = progress;
      }),
    );

    setState(() {
      _isUpdating = false;
    });
  }
}
