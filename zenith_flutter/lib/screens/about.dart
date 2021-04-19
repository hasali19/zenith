import 'package:flutter/material.dart';

class AboutScreen extends StatelessWidget {
  final String commitHash;

  AboutScreen({@required this.commitHash});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("About"),
      ),
      body: Center(
        child: Text(commitHash ?? "No commit hash"),
      ),
    );
  }
}
