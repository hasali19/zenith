import 'package:flutter/material.dart';
import 'package:package_info_plus/package_info_plus.dart';
import 'package:zenith/update_dialog.dart';
import 'package:zenith/updater.dart';

class SettingsScreen extends StatefulWidget {
  const SettingsScreen({Key? key}) : super(key: key);

  @override
  State<SettingsScreen> createState() => _SettingsScreenState();
}

class _SettingsScreenState extends State<SettingsScreen> {
  final _updater = Updater();

  PackageInfo? _packageInfo;

  @override
  void initState() {
    super.initState();
    () async {
      final info = await PackageInfo.fromPlatform();
      setState(() {
        _packageInfo = info;
      });
    }();
  }

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
          title: const Text("Version"),
          subtitle: Text(_packageInfo?.version ?? ""),
        ),
        ListTile(
          title: const Text("Build number"),
          subtitle: Text(_packageInfo?.buildNumber ?? ""),
        ),
        ListTile(
          title: const Text("Commit"),
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
