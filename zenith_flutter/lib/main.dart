import 'package:flutter/material.dart';
import 'package:provider/provider.dart';
import 'package:zenith/api.dart';
import 'package:zenith/screens/main/main.dart';

void main() {
  runApp(App());
}

class App extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final pageTransitionsTheme = PageTransitionsTheme(
      builders: {
        TargetPlatform.android: ZoomPageTransitionsBuilder(),
      },
    );

    return Provider<ApiClient>(
      create: (context) => ApiClient('https', 'zenith.hasali.uk', 443),
      child: MaterialApp(
        title: 'Zenith',
        themeMode: ThemeMode.system,
        theme: ThemeData.light().copyWith(
          pageTransitionsTheme: pageTransitionsTheme,
        ),
        darkTheme: ThemeData.dark().copyWith(
          pageTransitionsTheme: pageTransitionsTheme,
        ),
        home: MainScreen(),
      ),
    );
  }
}
