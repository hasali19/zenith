import 'dart:io';

import 'package:cookie_jar/cookie_jar.dart';
import 'package:dio/dio.dart';
import 'package:dio_cookie_manager/dio_cookie_manager.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';

CookieJar createCookieJar() {
  if (kIsWeb) {
    return _StubCookieJar();
  } else {
    final storage = kDebugMode && Platform.isWindows
        ? FileStorage()
        : _SecureCookieStorage();
    return PersistCookieJar(storage: storage);
  }
}

class _StubCookieJar implements CookieJar {
  @override
  Future<void> delete(Uri uri, [bool withDomainSharedCookie = false]) async {}

  @override
  Future<void> deleteAll() async {}

  @override
  bool get ignoreExpires => false;

  @override
  Future<List<Cookie>> loadForRequest(Uri uri) async {
    return [];
  }

  @override
  Future<void> saveFromResponse(Uri uri, List<Cookie> cookies) async {}
}

Dio createDioClient(String baseUrl, CookieJar cookieJar) {
  final dio = Dio(BaseOptions(
    baseUrl: baseUrl,
    extra: {'withCredentials': true},
  ));

  if (!kIsWeb) {
    dio.interceptors.add(CookieManager(cookieJar));
  }

  return dio;
}

class _SecureCookieStorage implements Storage {
  final _store = const FlutterSecureStorage(
    aOptions: AndroidOptions(encryptedSharedPreferences: true),
  );

  @override
  Future<void> init(bool persistSession, bool ignoreExpires) async {}

  @override
  Future<void> delete(String key) {
    return _store.delete(key: key);
  }

  @override
  Future<void> deleteAll(List<String> keys) {
    return _store.deleteAll();
  }

  @override
  Future<String?> read(String key) {
    return _store.read(key: key);
  }

  @override
  Future<void> write(String key, String value) {
    return _store.write(key: key, value: value);
  }
}
