import 'dart:async';
import 'dart:io';

import 'package:calc_manager/calc_manager.dart';
import 'dart:isolate';

void main() async {
  Programmer programmer = Programmer();
  final controller = StreamController<int>();
  final stream = programmer.commands(controller.stream);

  int value = 42;

  await for (var data in stream) {
    print('from server: $data');
    controller.add(value);
    value += 1;
  }

  controller.close();

  programmer.dispose();
}