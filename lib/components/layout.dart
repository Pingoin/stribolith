import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:stribolith/router.dart';

class AppLayout extends StatefulWidget {
  const AppLayout(this.child, {super.key});

  final Widget child;

  @override
  State<AppLayout> createState() => _AppLayoutState();
}

class _AppLayoutState extends State<AppLayout> {
  int selectedIndex = 0;
  @override
  Widget build(BuildContext context) {
    final theme = Theme.of(context);
    final style = theme.textTheme.labelSmall!.copyWith(
      color: theme.colorScheme.tertiary,
    );
    return LayoutBuilder(
      builder: (context, constraints) {
        return Scaffold(
          body: Row(
            children: [
              SafeArea(
                child: SingleChildScrollView(
                  child: ConstrainedBox(
                    constraints: BoxConstraints(
                      minHeight: MediaQuery.of(context).size.height,
                    ),
                    child: IntrinsicHeight(
                      child: navigation(constraints, style, context),
                    ),
                  ),
                ),
              ),
              Expanded(
                child: Container(
                  color: Theme.of(context).colorScheme.primaryContainer,
                  child: SingleChildScrollView(
                    child: ConstrainedBox(child: widget.child,
                    constraints: BoxConstraints(
                      minHeight: MediaQuery.of(context).size.height,
                    ),),
                    
                  ),
                ),
              ),
            ],
          ),
        );
      },
    );
  }

  NavigationRail navigation(
    BoxConstraints constraints,
    TextStyle style,
    BuildContext context,
  ) {
    return NavigationRail(
      extended: constraints.maxWidth >= 600,
      unselectedLabelTextStyle: style,
      destinations:
          baseRouteDestinations.map((el) => el.toNavRailDest()).toList(),
      selectedIndex: selectedIndex,
      onDestinationSelected: (value) {
        setState(() {
          selectedIndex = value;
          context.go(baseRouteDestinations[selectedIndex].path);
        });
      },
    );
  }
}
