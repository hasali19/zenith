import 'package:plugin_platform_interface/plugin_platform_interface.dart';

abstract class CastFrameworkPlatform extends PlatformInterface {
  CastFrameworkPlatform() : super(token: _token);

  static final Object _token = Object();

  static CastFrameworkPlatform _instance = _UnsupportCastFrameworkPlatform();

  static CastFrameworkPlatform get instance => _instance;

  static set instance(CastFrameworkPlatform instance) {
    PlatformInterface.verifyToken(instance, _token);
    _instance = instance;
  }
}

class _UnsupportCastFrameworkPlatform extends CastFrameworkPlatform {}
