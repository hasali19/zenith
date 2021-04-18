import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'package:url_launcher/url_launcher.dart';

void main() {
  runApp(App());
}

class App extends StatefulWidget {
  @override
  State<StatefulWidget> createState() {
    return AppState();
  }
}

const GIT_COMMIT_HASH = bool.hasEnvironment('GIT_COMMIT_HASH')
    ? String.fromEnvironment('GIT_COMMIT_HASH')
    : null;

const UPDATE_URL =
    'https://nightly.link/hasali19/zenith/workflows/flutter/flutter/zenith-apk.zip';

Future _checkForUpdates() async {
  if (GIT_COMMIT_HASH == null) {
    return;
  }

  final path = 'repos/hasali19/zenith/actions/workflows/8229171/runs';
  final uri = Uri.https('api.github.com', path, {'per_page': "1"});
  final res = await http.get(uri);

  final Iterable runs = jsonDecode(res.body)['workflow_runs'];
  final run = runs.firstWhere(
      (run) => run['status'] == 'completed' && run['conclusion'] == 'success');

  final hash = run['head_sha'];
  if (hash != GIT_COMMIT_HASH) {
    await canLaunch(UPDATE_URL)
        ? await launch(UPDATE_URL)
        : throw 'Could not launch $UPDATE_URL';
  }
}

class AppState extends State<App> {
  Future _future;

  @override
  void initState() {
    super.initState();
    _future = _checkForUpdates();
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Zenith',
      home: Scaffold(
        body: Center(
          child: FutureBuilder(
            future: _future,
            builder: (context, snapshot) {
              if (snapshot.connectionState == ConnectionState.done) {
                return Text("Hello, world!");
              } else if (snapshot.hasError) {
                return Text("${snapshot.error}");
              } else {
                return CircularProgressIndicator();
              }
            },
          ),
        ),
      ),
    );
  }
}
