import 'package:http/browser_client.dart';
import 'package:http/http.dart';

Client createHttpClient() => BrowserClient()..withCredentials = true;
