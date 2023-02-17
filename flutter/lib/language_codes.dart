import 'dart:convert';

import 'package:flutter/services.dart' show rootBundle;

final Map<String, String> _languageMap = {};

Future loadLanguageCodes() async {
  final List<dynamic> data =
      jsonDecode(await rootBundle.loadString("assets/language-codes.json"));
  for (final language in data) {
    final String names = language['English'];
    final name = names.split(";").first.trim();
    _languageMap[language['alpha3-b']] = name;
    _languageMap[language['alpha2']] = name;
  }
}

String tryResolveLanguageCode(String code) {
  return _languageMap[code] ?? code;
}
