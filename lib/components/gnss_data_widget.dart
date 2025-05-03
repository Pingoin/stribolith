import 'package:flutter/material.dart';
import 'package:stribolith/src/bindings/bindings.dart';

class GnssDataWidget extends StatelessWidget {
  final GnssData data;

  const GnssDataWidget({Key? key, required this.data}) : super(key: key);

  String _modeToString(int mode) {
    switch (mode) {
      case 0:
        return 'No Fix';
      case 1:
        return '2D Fix';
      case 2:
        return '3D Fix';
    }
    return 'No Fix';
  }

  String _systemToString(int system) {
    switch (system) {
      case 0:
        return 'GPS';
      case 1:
        return 'SBAS';
      case 2:
        return 'Galileo';
      case 3:
        return 'Beidou';
      case 4:
        return 'IMES';
      case 5:
        return 'QZSS';
      case 6:
        return 'GLONASS';
      case 7:
        return 'IRNSS';
    }
    return "";
  }

  @override
  Widget build(BuildContext context) {
    return SingleChildScrollView(
      child: Column(
        children: [
          Card(
            margin: const EdgeInsets.all(8),
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    'Position: ${data.lat}, ${data.lon}, Alt: ${data.alt.toStringAsPrecision(4)} m',
                  ),
                  Text('Leap Seconds: ${data.leapSeconds}'),
                  Text(
                    'Errors: Lon ${data.estimatedErrorLongitude.toStringAsPrecision(4)} m, Lat ${data.estimatedErrorLatitude.toStringAsPrecision(4)} m',
                  ),
                  Text(
                    'Plane Error: ${data.estimatedErrorPlane.toStringAsPrecision(4)} m, Altitude Error: ${data.estimatedErrorAltitude.toStringAsPrecision(4)} m',
                  ),
                  Text('Track: ${data.track.toStringAsPrecision(4)}°'),
                  Text('Speed: ${data.speed.toStringAsPrecision(4)} m/s'),
                  Text('Climb: ${data.climb.toStringAsPrecision(4)} m/s'),
                  Text('Mode: ${_modeToString(data.mode)}'),
                  Text(
                    'Error (Track/Speed/Climb): ${data.estimatedErrorTrack.toStringAsPrecision(4)}/${data.estimatedErrorSpeed.toStringAsPrecision(4)}/${data.estimatedErrorClimb.toStringAsPrecision(4)}',
                  ),
                ],
              ),
            ),
          ),
          Card(
            margin: const EdgeInsets.all(8),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const Padding(
                  padding: EdgeInsets.all(16.0),
                  child: Text(
                    'Satellites',
                    style: TextStyle(fontSize: 18, fontWeight: FontWeight.bold),
                  ),
                ),
                SizedBox(
                  height: 400,
                  child: SingleChildScrollView(
                    scrollDirection:
                        Axis.vertical, // Tabelle kann horizontal scrollen, falls nötig
                    child: DataTable(
                      columns: const [
                        DataColumn(label: Text('PRN')),
                        DataColumn(label: Text('Elevation')),
                        DataColumn(label: Text('Azimuth')),
                        DataColumn(label: Text('Signal')),
                        DataColumn(label: Text('Used')),
                        DataColumn(label: Text('System')),
                      ],
                      rows:
                          data.satellites
                              .map(
                                (sat) => DataRow(
                                  cells: [
                                    DataCell(Text('${sat.prn}')),
                                    DataCell(
                                      Text(
                                        '${sat.elevation.toStringAsFixed(1)}°',
                                      ),
                                    ),
                                    DataCell(
                                      Text(
                                        '${sat.azimuth.toStringAsFixed(1)}°',
                                      ),
                                    ),
                                    DataCell(
                                      Text(
                                        '${sat.signalStrength.toStringAsFixed(1)} dBHz',
                                      ),
                                    ),
                                    DataCell(
                                      Icon(
                                        sat.used
                                            ? Icons.check_circle
                                            : Icons.cancel,
                                        color:
                                            sat.used
                                                ? Colors.green
                                                : Colors.red,
                                        size: 20,
                                      ),
                                    ),
                                    DataCell(Text(_systemToString(sat.system))),
                                  ],
                                ),
                              )
                              .toList(),
                    ),
                  ),
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
