import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:stribolith/router.dart';
import 'package:stribolith/src/bindings/bindings.dart';

class Piscope extends StatelessWidget {
  final Widget? child;
  final List<Destination> children;
  const Piscope(this.child, this.children,{super.key});

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        ElevatedButton(onPressed: (){
          
          ConnectOpenPiScope(
            host:"192.168.178.84"
          ).sendSignalToRust();
        }, child: Text("Connect")),
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

