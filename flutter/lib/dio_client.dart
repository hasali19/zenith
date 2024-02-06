import 'package:cookie_jar/cookie_jar.dart';
import 'package:dio/dio.dart';
import 'package:dio_cookie_manager/dio_cookie_manager.dart';
import 'package:flutter/foundation.dart';

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
