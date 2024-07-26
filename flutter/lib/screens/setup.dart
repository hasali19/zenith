import 'dart:math';

import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:uuid/uuid.dart';
import 'package:zenith/main_router.dart';
import 'package:zenith/preferences.dart';
import 'package:zenith/responsive.dart';
import 'package:zenith/router/stack_router.dart';

const _uuid = Uuid();

class SetupScreen extends ConsumerStatefulWidget {
  const SetupScreen({super.key});

  @override
  ConsumerState<ConsumerStatefulWidget> createState() => _SetupScreenState();
}

class _SetupScreenState extends ConsumerState<SetupScreen> {
  final _key = GlobalKey<FormState>();

  var _scheme = 'http';
  final _nameController = TextEditingController();
  final _hostController = TextEditingController();
  final _portController = TextEditingController(text: '8000');

  @override
  void dispose() {
    _nameController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    Widget content = Form(
      key: _key,
      child: Column(
        children: [
          TextFormField(
            controller: _nameController,
            decoration: const InputDecoration(hintText: 'Server name'),
          ),
          const SizedBox(height: 8),
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
                    if (_scheme == 'https') {
                      _portController.text = '443';
                    }
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
              Padding(
                padding:
                    EdgeInsets.fromLTRB(4, context.isDesktop ? 12 : 14, 4, 0),
                child: const Text(':'),
              ),
              SizedBox(
                width: 100,
                child: TextFormField(
                  controller: _portController,
                  keyboardType: TextInputType.number,
                  validator: (value) {
                    final port = int.tryParse(value ?? '');
                    if (port == null || port < 0 || port > pow(2, 16)) {
                      return 'Invalid port number';
                    } else {
                      return null;
                    }
                  },
                ),
              ),
            ],
          ),
          const SizedBox(height: 32),
          ElevatedButton(
            child: const Text('Continue'),
            onPressed: () async {
              if (_key.currentState?.validate() != true) {
                return;
              }

              final servers = ref.read(serversPrefProvider);
              final server = Server(
                id: _uuid.v4(),
                name: _nameController.text,
                url: Uri(
                  scheme: _scheme,
                  host: _hostController.text,
                  port: int.parse(_portController.text),
                ).toString(),
              );

              await ref
                  .read(serversPrefProvider.notifier)
                  .update([...servers, server]);

              if (context.mounted) {
                StackRouter.of<PrimaryRoute>(context)
                    .replace(const LoginRoute(redirect: null));
              }
            },
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
        title: const Text('Add server'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: content,
      ),
    );
  }
}
