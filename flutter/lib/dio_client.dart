import 'package:cookie_jar/cookie_jar.dart';
import 'package:dio/dio.dart';
import 'package:dio_cookie_manager/dio_cookie_manager.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';

Dio createDioClient(String baseUrl) {
  final dio = Dio(BaseOptions(
    baseUrl: baseUrl,
    extra: {'withCredentials': true},
  ));

  if (!kIsWeb) {
    final storage = kDebugMode ? FileStorage() : _SecureCookieStorage();
    dio.interceptors.add(CookieManager(PersistCookieJar(storage: storage)));
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
