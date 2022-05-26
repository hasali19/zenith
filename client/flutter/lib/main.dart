import 'package:flutter/material.dart';
import 'package:zenith_flutter/api.dart';
import 'package:zenith_flutter/drawer.dart';
import 'package:zenith_flutter/screens/media_library.dart';
import 'package:zenith_flutter/screens/show_details.dart';
import 'package:zenith_flutter/screens/video_player.dart';

void main() {
  runApp(const ZenithApp());
}

class ZenithApp extends StatelessWidget {
  const ZenithApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    const pageTransitionsTheme = PageTransitionsTheme(builders: {
      TargetPlatform.android: ZoomPageTransitionsBuilder(),
    });

    return MaterialApp(
      title: 'Zenith',
      theme: ThemeData(
        primarySwatch: Colors.blue,
        pageTransitionsTheme: pageTransitionsTheme,
      ),
      darkTheme: ThemeData.dark().copyWith(
        scaffoldBackgroundColor: const Color.fromARGB(255, 36, 36, 36),
        pageTransitionsTheme: pageTransitionsTheme,
        useMaterial3: true,
      ),
      home: const MainScreen(),
    );
  }
}

class MainScreen extends StatefulWidget {
  const MainScreen({Key? key}) : super(key: key);
  @override
  State<MainScreen> createState() => _MainScreenState();
}

enum Screen {
  home,
  movies,
  shows,
  settings,
}

class _MainScreenState extends State<MainScreen> {
  Screen _screen = Screen.home;

  @override
  Widget build(BuildContext context) {
    final desktop = MediaQuery.of(context).size.width > 960;

    // Use a permanent navigation drawer on larger screens

    final drawer = NavigationDrawer(
      current: _screen,
      onTap: (screen) {
        if (!desktop) {
          Navigator.pop(context);
        }
        setState(() => _screen = screen);
      },
    );

    if (desktop) {
      return Row(
        children: [
          drawer,
          Expanded(
            child: Scaffold(
              body: _buildScreen(_screen),
            ),
          ),
        ],
      );
    } else {
      return Scaffold(
        appBar: AppBar(
          title: Text(_title(_screen)),
        ),
        drawer: desktop ? null : drawer,
        body: _buildScreen(_screen),
      );
    }
  }

  String _title(Screen screen) {
    switch (screen) {
      case Screen.home:
        return "Zenith";

      case Screen.movies:
        return "Movies";

      case Screen.shows:
        return "Shows";

      case Screen.settings:
        return "Settings";
    }
  }

  Widget _buildScreen(Screen screen) {
    switch (screen) {
      case Screen.home:
        return const Center(
          child: Icon(Icons.home),
        );

      case Screen.movies:
        return MediaLibraryScreen(
          key: const ValueKey(Screen.movies),
          provider: fetchMovies,
          onItemTap: (item) {
            Navigator.push(
              context,
              MaterialPageRoute(
                builder: (context) => VideoPlayerScreen(id: item.id),
              ),
            );
          },
        );

      case Screen.shows:
        return MediaLibraryScreen(
          key: const ValueKey(Screen.shows),
          provider: fetchShows,
          onItemTap: (item) {
            Navigator.push(
              context,
              MaterialPageRoute(
                builder: (context) => ShowDetailsScreen(show: item as Show),
              ),
            );
          },
        );

      case Screen.settings:
        return const Center(
          child: Icon(Icons.settings),
        );
    }
  }
}
