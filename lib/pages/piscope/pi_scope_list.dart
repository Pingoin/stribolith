import 'package:flutter/material.dart';
import 'package:stribolith/src/bindings/signals/signals.dart';

class PiScopeList extends StatelessWidget {
  const PiScopeList({
    super.key,
  });

  @override
  Widget build(BuildContext context) {
    return StreamBuilder(
      stream: PiScopeServerList.rustSignalStream,
      builder: (context, snapshot) {
        final signalPack = snapshot.data;
        if (signalPack == null) {
          return Placeholder();
        } else {
          PiScopeServerList message = signalPack.message;
          return Column(
            children: 
              message.servers.map((serv) =>Card(child: ElevatedButton(
                child: Text("connect to: ${serv.host}"),
                onPressed: (){
                            ConnectOpenPiScope(
            host:serv.host
          ).sendSignalToRust();
                },
              ))).toList(),
          );
        }
      },
    );
  }
}
