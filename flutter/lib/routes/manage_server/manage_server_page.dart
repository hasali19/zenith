import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:zenith/router.dart';

@RoutePage()
class ManageServerPage extends StatelessWidget {
  const ManageServerPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Manage Server'),
      ),
      body: ListView(
        children: [
          ListTile(
            leading: const Icon(Icons.account_circle),
            title: const Text('Users'),
            onTap: () {
              context.router.push(const ManageUsersRoute());
            },
          ),
        ],
      ),
    );
  }
}
