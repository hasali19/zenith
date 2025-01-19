import 'package:flutter/services.dart';

import 'file_picker_api.dart';

const _channel = MethodChannel('zenith.hasali.dev/platform');

Future<FilePickerResult?> showFilePicker() async {
  final result = await _channel.invokeMapMethod('showFilePicker');
  if (result == null) {
    return null;
  }

  return FilePickerResult(
    path: result['path'],
    name: result['name'],
    bytes: result['bytes'],
  );
}
