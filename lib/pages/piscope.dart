import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:stribolith/router.dart';

class Piscope extends StatelessWidget {
  final Widget? child;
  final List<Destination> children;
  const Piscope(this.child, this.children,{super.key});

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        child == null ? Placeholder() : child!,
        Row(
          children: children.map((el)=> ElevatedButton(onPressed: (){
          context.go(el.path);
        }, child: Text(el.name))).toList(),
        )

      ],
    );
  }
}

