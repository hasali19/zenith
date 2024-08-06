import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:package_info_plus/package_info_plus.dart';
import 'package:zenith/preferences.dart';
import 'package:zenith/responsive.dart';
import 'package:zenith/updater.dart';

@RoutePage()
class SettingsScreen extends ConsumerStatefulWidget {
  const SettingsScreen({super.key});

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

  Future<void> _checkForUpdates(BuildContext context) async {
    final update = await _updater.checkForUpdates();
    if (!context.mounted) return;
    if (update != null) {
      await update.install();
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
    final desktop = context.isDesktop;
    return Scaffold(
      appBar: AppBar(
        title: const Text('Settings'),
      ),
      body: ListView(
        padding: desktop ? const EdgeInsets.symmetric(vertical: 16) : null,
        children: [
          Padding(
            padding: const EdgeInsets.symmetric(vertical: 4, horizontal: 16),
            child: Text(
              'Appearance',
              style: TextStyle(color: Theme.of(context).colorScheme.primary),
            ),
          ),
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
          Padding(
            padding: const EdgeInsets.symmetric(vertical: 4, horizontal: 16),
            child: Text(
              'Player',
              style: TextStyle(color: Theme.of(context).colorScheme.primary),
            ),
          ),
          ListTile(
            title: const Text('Fast forward duration'),
            subtitle:
                Text('${ref.watch(fastForwardDurationProvider).value} seconds'),
            onTap: () async {
              final duration =
                  await showModalBottomSheet<PlayerSeekPresetDuration>(
                context: context,
                clipBehavior: Clip.antiAlias,
                builder: (context) => SafeArea(
                  child: Wrap(
                    children: PlayerSeekPresetDuration.values
                        .map((e) => ListTile(
                              title: Text('${e.value} seconds'),
                              onTap: () => Navigator.pop(context, e),
                            ))
                        .toList(),
                  ),
                ),
              );
              if (duration != null) {
                ref.read(fastForwardDurationProvider.notifier).update(duration);
              }
            },
          ),
          ListTile(
            title: const Text('Rewind duration'),
            subtitle:
                Text('${ref.watch(rewindDurationProvider).value} seconds'),
            onTap: () async {
              final duration =
                  await showModalBottomSheet<PlayerSeekPresetDuration>(
                context: context,
                clipBehavior: Clip.antiAlias,
                builder: (context) => SafeArea(
                  child: Wrap(
                    children: PlayerSeekPresetDuration.values
                        .map((e) => ListTile(
                              title: Text('${e.value} seconds'),
                              onTap: () => Navigator.pop(context, e),
                            ))
                        .toList(),
                  ),
                ),
              );
              if (duration != null) {
                ref.read(rewindDurationProvider.notifier).update(duration);
              }
            },
          ),
          CheckboxListTile(
            title: const Text('Set watched on skip'),
            subtitle: const Text(
                'When skipping forward in the playlist, mark the current video as watched'),
            value: ref.watch(setWatchedOnSkipProvider),
            onChanged: (value) {
              if (value != null) {
                ref.read(setWatchedOnSkipProvider.notifier).update(value);
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
        ],
      ),
    );
  }
}
