/// This file shims dart:ui in web-only scenarios, getting rid of the need to
/// suppress analyzer warnings.
///
// TODO(flutter/flutter#55000) Remove this file once web-only dart:ui APIs
// are exposed from a dedicated place.
export 'dart_ui_fake.dart' if (dart.library.html) 'dart_ui_real.dart';
