import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:stribolith/components/layout.dart';
import 'package:stribolith/pages/piscope.dart';
import 'package:stribolith/pages/piscope/gnss_raw.dart';
import 'package:stribolith/pages/piscope/orientation_raw.dart';

// GoRouter configuration
final router = GoRouter(
  initialLocation: '/',
  navigatorKey: _rootNavigatorKey,
  routes: [
    ShellRoute(
      navigatorKey: _shellNavigatorKey,
      builder: (context, state, child) {
        return AppLayout(child);
      },
      routes: baseRouteDestinations.map((el) => el.toGoRoute()).toList(),
    ),
  ],
);

final _rootNavigatorKey = GlobalKey<NavigatorState>();
final _shellNavigatorKey = GlobalKey<NavigatorState>();

final baseRouteDestinations = [
  Destination(
    name:
        'Piscope',
    path: '/',
    page: (context, state, child, children) {
      return Piscope(child, children);
    },
    icon: Icon(Icons.rocket),
    children: [
      Destination(
        name: "Gnss",
        path: "/gnss",
        page: (context, state, child, children) {
          return GnssRaw();
        },
        children: [],
        icon: Icon(Icons.gps_fixed),
      ),
            Destination(
        name: "Orientation",
        path: "/",
        page: (context, state, child, children) {
          return OrientationRaw();
        },
        children: [],
        icon: Icon(Icons.gps_fixed),
      ),
    ],
  ),
];

class Destination {
  String name;
  String path;
  Widget Function(BuildContext, GoRouterState, Widget?, List<Destination>) page;
  Widget icon;
  GlobalKey<NavigatorState>? parentNavigationKey;
  List<Destination> children;

  Destination({
    required this.name,
    required this.path,
    required this.page,
    required this.icon,
    required this.children,
    this.parentNavigationKey,
  });

  RouteBase toGoRoute() {
    if (children.isEmpty) {
      return GoRoute(
        path: path,
        name: name,
        builder: (context, state) => page(context, state, null, []),
        parentNavigatorKey: parentNavigationKey,
      );
    } else {
      return ShellRoute(
        routes: children.map((el) => el.toGoRoute()).toList(),
        builder:
            (context, state, child) => page(context, state, child, children),
      );
    }
  }

  NavigationRailDestination toNavRailDest() {
    return NavigationRailDestination(icon: icon, label: Text(name));
  }
}
