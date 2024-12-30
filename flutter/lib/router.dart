import 'package:auto_route/auto_route.dart';
import 'package:cast_framework/cast_framework.dart' hide MediaType;
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:zenith/api.dart';
import 'package:zenith/constants.dart';
import 'package:zenith/image.dart';
import 'package:zenith/main.dart';
import 'package:zenith/media_route_button/media_route_button.dart';
import 'package:zenith/responsive.dart';
import 'package:zenith/routes/routes.dart';
import 'package:zenith/screens/home.dart';
import 'package:zenith/screens/media_library.dart';
import 'package:zenith/screens/settings.dart';
import 'package:zenith/screens/setup.dart';
import 'package:zenith/screens/video_player.dart';

part 'router.gr.dart';

@AutoRouterConfig()
class AppRouter extends RootStackRouter {
  final Future<bool> Function() isServerSet;
  final Future<bool?> Function() isLoggedIn;

  AppRouter({required this.isServerSet, required this.isLoggedIn});

  @override
  RouteType get defaultRouteType => const RouteType.material();

  @override
  List<AutoRoute> get routes => [
        AutoRoute(
          path: '/',
          page: MainRoute.page,
          initial: true,
          guards: [ServerSetupGuard(isServerSet), AuthGuard(isLoggedIn)],
          children: [
            AutoRoute(
              page: LibraryRoute.page,
              initial: true,
              children: [
                AutoRoute(
                  page: LibraryTabsRoute.page,
                  initial: true,
                  children: [
                    AutoRoute(page: HomeRoute.page, initial: true),
                    AutoRoute(path: 'movies', page: MoviesRoute.page),
                    AutoRoute(path: 'shows', page: ShowsRoute.page),
                  ],
                ),
                AutoRoute(
                  path: 'items/:id',
                  page: ItemDetailsRoute.page,
                  usesPathAsKey: true,
                ),
              ],
            ),
            AutoRoute(
              path: 'server',
              page: ManageServerShellRoute.page,
              children: [
                AutoRoute(page: ManageServerRoute.page, initial: true),
                AutoRoute(path: 'users', page: ManageUsersRoute.page),
              ],
            ),
            AutoRoute(path: 'settings', page: SettingsRoute.page),
          ],
        ),
        AutoRoute(
          path: '/player/:id',
          page: VideoPlayerRoute.page,
          usesPathAsKey: true,
          guards: [ServerSetupGuard(isServerSet), AuthGuard(isLoggedIn)],
        ),
        AutoRoute(
          path: '/login',
          page: LoginRoute.page,
          guards: [ServerSetupGuard(isServerSet)],
          children: [
            AutoRoute(page: LoginUsersRoute.page, initial: true),
            AutoRoute(path: 'user', page: LoginUserRoute.page),
            AutoRoute(path: 'register', page: LoginRegisterRoute.page),
          ],
        ),
        AutoRoute(path: '/setup', page: SetupRoute.page),
      ];
}

@RoutePage()
class ManageServerShellPage extends StatelessWidget {
  const ManageServerShellPage({super.key});

  @override
  Widget build(BuildContext context) {
    return AutoRouter();
  }
}

class ServerSetupGuard extends AutoRouteGuard {
  final Future<bool> Function() isServerSet;

  ServerSetupGuard(this.isServerSet);

  @override
  void onNavigation(NavigationResolver resolver, StackRouter router) async {
    if (await isServerSet()) {
      return resolver.next(true);
    }

    router.replace(const SetupRoute());
  }
}

class AuthGuard extends AutoRouteGuard {
  final Future<bool?> Function() isLoggedIn;

  AuthGuard(this.isLoggedIn);

  @override
  void onNavigation(NavigationResolver resolver, StackRouter router) async {
    if (await isLoggedIn() != false) {
      return resolver.next(true);
    }

    router.replace(LoginRoute(redirect: resolver.route.stringMatch));
  }
}

@RoutePage()
class LibraryPage extends StatelessWidget {
  const LibraryPage({super.key});

  @override
  Widget build(BuildContext context) {
    return AutoRouter();
  }
}

@RoutePage()
class LibraryTabsPage extends StatelessWidget {
  const LibraryTabsPage({super.key});

  @override
  Widget build(BuildContext context) {
    return AutoTabsRouter(
      routes: const [
        HomeRoute(),
        MoviesRoute(),
        ShowsRoute(),
      ],
      builder: (context, child) {
        if (context.isDesktop) {
          return child;
        }

        return Column(
          children: [
            AppBar(
              title: Text('Zenith'),
              actions: [
                _SearchButton(),
                if (CastFrameworkPlatform.instance.isSupported)
                  const MediaRouteButton(),
                PopupMenuButton(
                  itemBuilder: (context) {
                    return [
                      PopupMenuItem(
                        onTap: () {},
                        child: const Text('Logout'),
                      ),
                    ];
                  },
                ),
              ],
            ),
            Expanded(child: child),
          ],
        );
      },
    );
  }
}

class _SearchButton extends ConsumerStatefulWidget {
  const _SearchButton();

  @override
  ConsumerState<_SearchButton> createState() => _SearchButtonState();
}

class _SearchButtonState extends ConsumerState<_SearchButton> {
  String? _query;
  Iterable<Widget> _results = <Widget>[];

  @override
  Widget build(BuildContext context) {
    return SearchAnchor(
      builder: (context, controller) {
        return IconButton(
          icon: Icon(Icons.search),
          onPressed: () {
            controller.openView();
          },
        );
      },
      suggestionsBuilder: (context, controller) async {
        _query = controller.text;

        if (controller.text.length < 3) {
          return [];
        }

        final api = ref.read(apiProvider);
        final results = (await api
            .searchByName(_query!, types: [MediaType.movie, MediaType.show]));

        // If another search happened after this one, throw away these options.
        // Use the previous options instead and wait for the newer request to
        // finish.
        if (_query != controller.text) {
          return _results;
        }

        _results = List<ListTile>.generate(results.length, (int index) {
          final item = results[index];
          return ListTile(
            title: Text(item.name),
            subtitle: Text(item.startDate?.year.toString() ?? ''),
            minVerticalPadding: 24,
            leading: Card(
              margin: EdgeInsets.zero,
              clipBehavior: Clip.antiAlias,
              shape: RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(4)),
              child: ZenithApiImage(
                id: item.poster!,
                requestWidth: mediaPosterImageWidth,
              ),
            ),
            onTap: () {
              context.router.push(ItemDetailsRoute(id: item.id));
            },
          );
        });

        return _results;
      },
    );
  }
}

final _moviesProvider = FutureProvider((ref) async {
  final api = ref.watch(apiProvider);
  final movies = await api.fetchMovies();
  return movies.map((e) => MediaLibraryItem.fromMediaItem(e, api)).toList();
});

@RoutePage()
class MoviesScreen extends ConsumerWidget {
  const MoviesScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return MediaLibraryScreen(
      provider: _moviesProvider,
      posterFallback: Icons.movie,
      onRefresh: () => ref.refresh(_moviesProvider.future),
      onItemTap: (item) => context.router.push(ItemDetailsRoute(id: item.id)),
    );
  }
}

final _showsProvider = FutureProvider((ref) async {
  final api = ref.watch(apiProvider);
  final shows = await api.fetchShows();
  return shows.map((e) => MediaLibraryItem.fromMediaItem(e, api)).toList();
});

@RoutePage()
class ShowsScreen extends ConsumerWidget {
  const ShowsScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return MediaLibraryScreen(
      provider: _showsProvider,
      posterFallback: Icons.tv,
      onRefresh: () => ref.refresh(_showsProvider.future),
      onItemTap: (item) => context.router.push(ItemDetailsRoute(id: item.id)),
    );
  }
}
