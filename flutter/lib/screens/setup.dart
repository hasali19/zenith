import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:gap/gap.dart';
import 'package:uuid/uuid.dart';
import 'package:zenith/preferences.dart';
import 'package:zenith/responsive.dart';
import 'package:zenith/router.dart';

const _uuid = Uuid();

@RoutePage()
class SetupScreen extends ConsumerStatefulWidget {
  const SetupScreen({super.key});

  @override
  ConsumerState<ConsumerStatefulWidget> createState() => _SetupScreenState();
}

class _SetupScreenState extends ConsumerState<SetupScreen> {
  final _key = GlobalKey<FormState>();

  var _scheme = 'https';
  final _hostController = TextEditingController();

  @override
  void dispose() {
    super.dispose();
    _hostController.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final servers = ref.watch(serversPrefProvider);
    Widget content = Form(
      key: _key,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Row(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              SizedBox(
                width: 100,
                child: DropdownButtonFormField<String>(
                  items: const [
                    DropdownMenuItem(value: 'http', child: Text('http')),
                    DropdownMenuItem(value: 'https', child: Text('https')),
                  ],
                  value: _scheme,
                  onChanged: (value) => setState(() {
                    _scheme = value!;
                  }),
                ),
              ),
              const SizedBox(width: 16),
              Flexible(
                flex: 1,
                child: TextFormField(
                  controller: _hostController,
                  keyboardType: TextInputType.url,
                  decoration: const InputDecoration(
                    hintText: 'Host name or IP address',
                  ),
                  validator: (value) => value == null || value.isEmpty
                      ? 'This is required'
                      : null,
                ),
              ),
            ],
          ),
          const Gap(32),
          Align(
            alignment: Alignment.center,
            child: ElevatedButton(
              onPressed: _onSubmit,
              child: const Text('Continue'),
            ),
          ),
          if (servers.isNotEmpty)
            Padding(
              padding: const EdgeInsets.only(top: 32, bottom: 8),
              child: Text('Previously used',
                  style: TextStyle(fontStyle: FontStyle.italic)),
            ),
          for (final server in servers)
            ListTile(
              title: Text(server.url),
              onTap: () => _onSelectExisting(server.id),
            ),
        ],
      ),
    );

    if (context.isDesktop) {
      content = Center(
        child: ConstrainedBox(
          constraints: const BoxConstraints(
              maxWidth: 800, minHeight: 300, maxHeight: 300),
          child: Card(
            child: Padding(
              padding: const EdgeInsets.all(32),
              child: content,
            ),
          ),
        ),
      );
    }

    return Scaffold(
      appBar: AppBar(
        title: const Text('Select server'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: content,
      ),
    );
  }

  void _onSubmit() async {
    if (_key.currentState?.validate() != true) {
      return;
    }

    final url = '$_scheme://${_hostController.text}';
    final uri = Uri.tryParse(url);
    if (uri == null) {
      ScaffoldMessenger.of(context)
          .showSnackBar(SnackBar(content: Text('Invalid url: $url')));
    }

    final servers = ref.read(serversPrefProvider);
    final Server server;
    if (servers.where((server) => server.url == url).firstOrNull
        case Server existing) {
      server = existing;
    } else {
      server = Server(
        id: _uuid.v4(),
        name: null,
        url: url,
      );

      await ref.read(serversPrefProvider.notifier).update([server, ...servers]);
    }

    await ref.read(serverPrefProvider.notifier).update(server.id);

    if (mounted) {
      context.router.replaceAll([const MainRoute()]);
    }
  }

  void _onSelectExisting(String id) async {
    await ref.read(serverPrefProvider.notifier).update(id);

    if (mounted) {
      context.router.replaceAll([const MainRoute()]);
    }
  }
}
