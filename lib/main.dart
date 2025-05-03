import 'package:rinf/rinf.dart';
import 'package:stribolith/router.dart';
import 'src/bindings/bindings.dart';
import 'package:flutter/material.dart';

void main() async {
  await initializeRust(assignRustSignal);
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp.router(
      title: 'Flutter Demo',
      routerConfig: router,
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.blue),
      ),
    );
  }
}
