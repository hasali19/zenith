import 'dart:io';

import 'package:cookie_jar/cookie_jar.dart';
import 'package:dio/dio.dart';
import 'package:dio_cookie_manager/dio_cookie_manager.dart';
import 'package:flutter/foundation.dart';
import 'package:package_info_plus/package_info_plus.dart';
import 'package:zenith/updater.dart';

Dio createDioClient(
    String baseUrl, CookieJar cookieJar, PackageInfo packageInfo) {
  final dio = Dio(BaseOptions(
    baseUrl: baseUrl,
    extra: {'withCredentials': true},
    validateStatus: (status) =>
        status != null && (status >= 200 && status < 300) || status == 401,
    headers: {
      if (!kIsWeb)
        'User-Agent':
            'Flutter ${Platform.operatingSystem} ${packageInfo.buildNumber}/${Updater.revision?.substring(0, 7)}',
    },
    listFormat: ListFormat.multiCompatible,
  ));

  if (!kIsWeb) {
    dio.interceptors.add(CookieManager(cookieJar));
  }

  return dio;
}
