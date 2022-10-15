import 'package:flutter/material.dart';
import 'package:zenith_flutter/updater.dart';

class UpdateDialog extends StatefulWidget {
  const UpdateDialog({
    Key? key,
    required this.update,
  }) : super(key: key);

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
              const Expanded(child: Text("Downloading...")),
              Text("${_progress.toStringAsFixed(2)}M"),
            ],
          ),
          const SizedBox(height: 16),
          const LinearProgressIndicator(),
        ],
      );
    } else {
      content = const Text("An update is available");
    }

    return AlertDialog(
      title: const Text("Update"),
      content: content,
      actions: [
        TextButton(
          child: const Text("Cancel"),
          onPressed: !_isUpdating ? () => Navigator.pop(context) : null,
        ),
        TextButton(
          child: const Text("Apply"),
          onPressed: !_isUpdating ? _onApplyUpdate : null,
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
