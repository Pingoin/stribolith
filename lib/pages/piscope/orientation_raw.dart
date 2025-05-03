import 'package:flutter/material.dart';
import 'package:stribolith/src/bindings/bindings.dart';

class OrientationRaw extends StatelessWidget {
  const OrientationRaw({
    super.key,
  });

  @override
  Widget build(BuildContext context) {
    return StreamBuilder(
      stream: EulerAngle.rustSignalStream,
      builder: (context, snapshot) {
        final signalPack = snapshot.data;
        if (signalPack == null) {
          return Placeholder();
        } else {
          EulerAngle message = signalPack.message;
          return Text(message.pitch.toStringAsPrecision(6));
        }
      },
    );
  }
}
