import 'dart:typed_data';

class FilePickerResult {
  final String path;
  final String name;
  final Uint8List bytes;

  const FilePickerResult({
    required this.path,
    required this.name,
    required this.bytes,
  });
}

Future<FilePickerResult?> showFilePicker() {
  throw UnimplementedError();
}
