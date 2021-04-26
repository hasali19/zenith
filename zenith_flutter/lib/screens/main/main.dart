import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:http/http.dart' as http;
import 'package:zenith/screens/about.dart';

import 'home.dart';
import 'movies.dart';
import 'shows.dart';

const GIT_COMMIT_HASH = bool.hasEnvironment('GIT_COMMIT_HASH')
    ? String.fromEnvironment('GIT_COMMIT_HASH')
    : null;

const UPDATE_URL =
    'https://nightly.link/hasali19/zenith/workflows/flutter/flutter/zenith-apk.zip';

Future<bool> _checkForUpdates() async {
  if (GIT_COMMIT_HASH == null) {
    return false;
  }

  final path = 'repos/hasali19/zenith/actions/workflows/8229171/runs';
  final uri = Uri.https('api.github.com', path, {'per_page': '1'});
  final res = await http.get(uri);

  final Iterable runs = jsonDecode(res.body)['workflow_runs'];
  final run = runs.firstWhere(
      (run) => run['status'] == 'completed' && run['conclusion'] == 'success');

  final hash = run['head_sha'];
  if (hash != GIT_COMMIT_HASH) {
    return true;
  }

  return false;
}

class MainScreen extends StatefulWidget {
  @override
  State<StatefulWidget> createState() => MainScreenState();
}

class MainScreenState extends State<MainScreen> {
  final _screens = [
    () => HomeScreen(),
    () => MoviesScreen(),
    () => ShowsScreen(),
  ];

  var _current = 0;

  @override
  void initState() {
    super.initState();

    _checkForUpdates().then((value) {
      if (value) {
        showDialog(
          context: context,
          barrierDismissible: false,
          builder: (context) {
            return UpdateDialog();
          },
        );
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Zenith'),
        actions: [
          AppBarMenu(),
        ],
      ),
      body: AnimatedSwitcher(
        duration: const Duration(milliseconds: 200),
        child: _screens[_current](),
      ),
      bottomNavigationBar: BottomNavigationBar(
        items: [
          BottomNavigationBarItem(icon: Icon(Icons.home), label: 'Home'),
          BottomNavigationBarItem(icon: Icon(Icons.movie), label: 'Movies'),
          BottomNavigationBarItem(icon: Icon(Icons.tv), label: 'Shows'),
        ],
        currentIndex: _current,
        onTap: (item) => setState(() => _current = item),
      ),
    );
  }
}

class AppBarMenu extends StatelessWidget {
  void _handleMenuItemSelected(BuildContext context, String value) {
    if (value == 'about') {
      Navigator.push(
        context,
        MaterialPageRoute(
          builder: (context) => AboutScreen(commitHash: GIT_COMMIT_HASH),
        ),
      );
    }
  }

  @override
  Widget build(BuildContext context) {
    return PopupMenuButton(
      itemBuilder: (context) => [
        PopupMenuItem(
          value: 'about',
          child: Text('About'),
        ),
      ],
      onSelected: (String value) {
        _handleMenuItemSelected(context, value);
      },
    );
  }
}

class UpdateDialog extends StatefulWidget {
  @override
  State<StatefulWidget> createState() => UpdateDialogState();
}

class UpdateDialogState extends State<UpdateDialog> {
  static const platform = MethodChannel('zenith.hasali.uk/updater');

  var _updating = false;

  void _handleInstallTap() {
    if (!_updating) {
      platform.invokeMethod('installApk', <String, dynamic>{
        'url': UPDATE_URL,
      });

      setState(() => _updating = true);
    }
  }

  @override
  Widget build(BuildContext context) {
    var content;

    if (_updating) {
      content = Row(
        children: [
          CircularProgressIndicator(),
          SizedBox(width: 24),
          Text('Downloading'),
        ],
      );
    } else {
      content = Text('An update is available');
    }

    return AlertDialog(
      title: Text('Update'),
      content: content,
      actions: [
        TextButton(
          onPressed: _handleInstallTap,
          child: Text('Install'),
        ),
      ],
    );
  }
}
