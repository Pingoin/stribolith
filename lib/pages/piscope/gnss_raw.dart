import 'package:flutter/material.dart';
import 'package:stribolith/components/gnss_data_widget.dart';
import 'package:stribolith/src/bindings/bindings.dart';

class GnssRaw extends StatelessWidget {
  const GnssRaw({
    super.key,
  });

  @override
  Widget build(BuildContext context) {
    return StreamBuilder(
      stream: GnssData.rustSignalStream,
      builder: (context, snapshot) {
        final signalPack = snapshot.data;
        if (signalPack == null) {
          return Placeholder();
        } else {
          GnssData message = signalPack.message;
          return GnssDataWidget(data: message);
        }
      },
    );
  }
}
