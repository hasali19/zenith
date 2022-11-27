import 'package:flutter/material.dart';
import 'package:zenith/update_dialog.dart';
import 'package:zenith/updater.dart';

class SettingsScreen extends StatelessWidget {
  SettingsScreen({Key? key}) : super(key: key);

  final _updater = Updater();

  _checkForUpdates(BuildContext context) async {
    final update = await _updater.checkForUpdates();
    if (update != null) {
      showDialog(
        context: context,
        barrierDismissible: false,
        builder: (context) => UpdateDialog(update: update),
      );
    } else {
      ScaffoldMessenger.of(context).showSnackBar(SnackBar(
        content: const Text("No updates available"),
        action: SnackBarAction(
          label: "OK",
          onPressed: () {
            ScaffoldMessenger.of(context).hideCurrentSnackBar();
          },
        ),
      ));
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text("Settings")),
      body: ListView(children: [
        ListTile(
          title: const Text("Revision"),
          subtitle: Text(Updater.revision ?? "Unknown"),
        ),
        ListTile(
          title: const Text("Check for updates"),
          onTap: () {
            _checkForUpdates(context);
          },
        ),
      ]),
    );
  }
}
