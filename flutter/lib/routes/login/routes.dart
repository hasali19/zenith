sealed class LoginSubRoute {
  const LoginSubRoute();
}

class LoginUsersRoute extends LoginSubRoute {
  const LoginUsersRoute();
}

class LoginUserRoute extends LoginSubRoute {
  final String username;

  const LoginUserRoute({required this.username});
}

class LoginRegisterRoute extends LoginSubRoute {
  final bool initial;

  const LoginRegisterRoute({required this.initial});
}
