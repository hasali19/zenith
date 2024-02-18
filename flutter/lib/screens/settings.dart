import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:package_info_plus/package_info_plus.dart';
import 'package:zenith/preferences.dart';
import 'package:zenith/update_dialog.dart';
import 'package:zenith/updater.dart';

@RoutePage()
class SettingsScreen extends ConsumerStatefulWidget {
  const SettingsScreen({Key? key}) : super(key: key);

  @override
  ConsumerState<SettingsScreen> createState() => _SettingsScreenState();
}

class _SettingsScreenState extends ConsumerState<SettingsScreen> {
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
        content: const Text('No updates available'),
        action: SnackBarAction(
          label: 'OK',
          onPressed: () {
            ScaffoldMessenger.of(context).hideCurrentSnackBar();
          },
        ),
      ));
    }
  }

  @override
  Widget build(BuildContext context) {
    return ListView(children: [
      ListTile(
        title: const Text('Theme'),
        subtitle: Text(ref.watch(themeMode).label),
        onTap: () async {
          var selected = ref.read(themeMode);
          final updated = await showDialog(
            context: context,
            builder: (context) => AlertDialog(
              title: const Text('Choose theme'),
              content: StatefulBuilder(builder: (context, setState) {
                return Column(
                  mainAxisSize: MainAxisSize.min,
                  children: AppThemeMode.values
                      .map(
                        (value) => RadioListTile(
                          contentPadding: EdgeInsets.zero,
                          value: value,
                          title: Text(value.label),
                          groupValue: selected,
                          onChanged: (value) {
                            setState(() {
                              if (value != null) {
                                selected = value;
                              }
                            });
                          },
                        ),
                      )
                      .toList(),
                );
              }),
              actions: [
                TextButton(
                  onPressed: () => Navigator.pop(context),
                  child: const Text('Cancel'),
                ),
                ElevatedButton(
                  onPressed: () => Navigator.pop(context, selected),
                  child: const Text('Confirm'),
                ),
              ],
            ),
          );

          if (updated != null) {
            ref.read(themeMode.notifier).update(updated);
          }
        },
      ),
      CheckboxListTile(
        title: const Text('Use system colour scheme'),
        value: ref.watch(enableDynamicColor),
        onChanged: (value) {
          if (value != null) {
            ref.read(enableDynamicColor.notifier).update(value);
          }
        },
      ),
      const Divider(),
      ListTile(
        title: const Text('Version'),
        subtitle: Text(_packageInfo?.version ?? ''),
      ),
      ListTile(
        title: const Text('Build number'),
        subtitle: Text(_packageInfo?.buildNumber ?? ''),
      ),
      ListTile(
        title: const Text('Commit'),
        subtitle: Text(Updater.revision ?? 'Unknown'),
      ),
      CheckboxListTile(
        title: const Text('Auto update check'),
        subtitle: const Text('Notify if an update is available on startup'),
        value: ref.watch(enableUpdatesCheck),
        onChanged: (value) {
          if (value != null) {
            ref.read(enableUpdatesCheck.notifier).update(value);
          }
        },
      ),
      ListTile(
        title: const Text('Check for updates'),
        subtitle: const Text('Check immediately for available updates'),
        onTap: () {
          _checkForUpdates(context);
        },
      ),
    ]);
  }
}
