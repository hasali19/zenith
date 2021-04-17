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

Future fetchRuns() async {
  var res = await http.get(
    Uri.https(
      'api.github.com',
      'repos/hasali19/zenith/actions/workflows/8229171/runs',
      {'per_page': "1"},
    ),
  );

  final runs = jsonDecode(res.body);
  final run = runs['workflow_runs'][0];
  final runId = run['id'];
  final hash = run['head_sha'];
  final suiteId = run['check_suite_id'];

  if (hash != GIT_COMMIT_HASH) {
    res = await http.get(
      Uri.https(
        'api.github.com',
        'repos/hasali19/zenith/actions/runs/$runId/artifacts',
      ),
    );

    final artifacts = jsonDecode(res.body);
    final artifactId = artifacts['artifacts'][0]['id'];
    final url =
        'https://github.com/hasali19/zenith/suites/$suiteId/artifacts/$artifactId';

    await canLaunch(url) ? await launch(url) : throw 'Could not launch $url';
  }
}

class AppState extends State<App> {
  Future _runId;

  @override
  void initState() {
    super.initState();
    _runId = fetchRuns();
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Zenith',
      home: Scaffold(
        body: Center(
          child: FutureBuilder(
            future: _runId,
            builder: (context, snapshot) {
              if (snapshot.hasData) {
                return Text("${snapshot.data}");
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
