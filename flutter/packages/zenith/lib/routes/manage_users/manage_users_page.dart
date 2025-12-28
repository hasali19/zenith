import 'package:auto_route/auto_route.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:riverpod_annotation/riverpod_annotation.dart';
import 'package:share_plus/share_plus.dart';
import 'package:zenith/api.dart';

part 'manage_users_page.g.dart';

@riverpod
Future<({List<User> users, List<UserRegistration> registrations})> _data(
  Ref ref,
) async {
  final api = ref.watch(apiProvider);
  final [users, registrations] = await Future.wait([
    api.fetchUsers(),
    api.fetchUserRegistrations(),
  ]);
  return (
    users: users as List<User>,
    registrations: registrations as List<UserRegistration>,
  );
}

@RoutePage()
class ManageUsersPage extends ConsumerWidget {
  const ManageUsersPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final users = ref.watch(_dataProvider);
    return Scaffold(
      appBar: AppBar(title: const Text('Manage users')),
      body: switch (users) {
        AsyncData(value: (:final users, :final registrations)) => CustomScrollView(
          slivers: [
            SliverToBoxAdapter(
              child: Padding(
                padding: const EdgeInsets.symmetric(
                  vertical: 4,
                  horizontal: 16,
                ),
                child: Text(
                  'Users',
                  style: TextStyle(
                    color: Theme.of(context).colorScheme.primary,
                  ),
                ),
              ),
            ),
            SliverList.builder(
              itemCount: users.length,
              itemBuilder: (context, index) {
                final user = users[index];
                return ListTile(
                  leading: Icon(Icons.account_circle),
                  title: Text(user.username),
                );
              },
            ),
            const SliverToBoxAdapter(child: Divider()),
            SliverToBoxAdapter(
              child: Padding(
                padding: const EdgeInsets.symmetric(
                  vertical: 4,
                  horizontal: 16,
                ),
                child: Text(
                  'Registrations',
                  style: TextStyle(
                    color: Theme.of(context).colorScheme.primary,
                  ),
                ),
              ),
            ),
            SliverToBoxAdapter(
              child: ListTile(
                leading: const Icon(Icons.add_circle_outline),
                title: const Text('Create registration code'),
                onTap: () async {
                  final api = ref.read(apiProvider);
                  await api.createUserRegistration();
                  ref.invalidate(_dataProvider);
                },
              ),
            ),
            SliverList.builder(
              itemCount: registrations.length,
              itemBuilder: (context, index) {
                final registration = registrations[index];
                return Builder(
                  builder: (context) {
                    return GestureDetector(
                      child: ListTile(
                        leading: Icon(Icons.account_circle),
                        title: Text(registration.createdAt.toString()),
                        subtitle: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            Text.rich(
                              TextSpan(
                                children: [
                                  TextSpan(
                                    text: 'Expires: ',
                                    style: TextStyle(
                                      fontWeight: FontWeight.bold,
                                    ),
                                  ),
                                  TextSpan(
                                    text: registration.expiresAt.toString(),
                                  ),
                                ],
                              ),
                              maxLines: 1,
                              overflow: TextOverflow.ellipsis,
                            ),
                            Text(
                              registration.code,
                              maxLines: 1,
                              overflow: TextOverflow.ellipsis,
                            ),
                          ],
                        ),
                        isThreeLine: true,
                        trailing: IconButton(
                          icon: Icon(Icons.delete),
                          onPressed: () async {
                            await ref
                                .read(apiProvider)
                                .deleteUserRegistration(registration.code);
                            ref.invalidate(_dataProvider);
                          },
                        ),
                        onTap: () {
                          final api = ref.read(apiProvider);
                          final url =
                              '${api.baseUrl}/#/login/register?code=${registration.code}';
                          Share.share(url);
                        },
                      ),
                      onSecondaryTapUp: (details) {
                        final RenderBox box =
                            context.findRenderObject()! as RenderBox;
                        final RenderBox overlay =
                            Navigator.of(
                                  context,
                                ).overlay!.context.findRenderObject()!
                                as RenderBox;

                        var offset = box.localToGlobal(
                          details.localPosition,
                          ancestor: overlay,
                        );
                        showMenu(
                          context: context,
                          items: [
                            PopupMenuItem(
                              child: Text('Copy registration URL'),
                              onTap: () {
                                final api = ref.read(apiProvider);
                                final url =
                                    '${api.baseUrl}/#/login/register?code=${registration.code}';
                                Clipboard.setData(ClipboardData(text: url));
                              },
                            ),
                          ],
                          position: RelativeRect.fromRect(
                            Rect.fromPoints(offset, offset),
                            Offset.zero & overlay.size,
                          ),
                        );
                      },
                    );
                  },
                );
              },
            ),
          ],
        ),
        AsyncError(:final error) => Center(child: Text(error.toString())),
        _ => const Center(child: CircularProgressIndicator()),
      },
    );
  }
}
