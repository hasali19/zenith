import 'dart:convert';

import 'package:flutter/services.dart' show rootBundle;

final Map<String, String> _languageMap = {};

Future loadLanguageCodes() async {
  final List<dynamic> data =
      jsonDecode(await rootBundle.loadString("assets/language-codes.json"));
  for (final language in data) {
    String name = language['English'];
    _languageMap[language['alpha3-b']] = name.split(";").first.trim();
  }
}

String tryResolveLanguageCode(String code) {
  return _languageMap[code] ?? code;
}
