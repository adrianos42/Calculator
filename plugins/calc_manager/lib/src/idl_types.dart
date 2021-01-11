import 'idl_types_interface.dart' as idl_types_interface;
import 'idl_interface_constructor.dart' as idl_interface_constructor;
import 'dart:typed_data';
import 'dart:collection';
import 'package:ffi_internal/ffi_internal.dart';

enum Command {
  hex,
  dec,
  oct,
  bin,
  qword,
  dword,
  word,
  byte,
}

class Programmer implements Disposable {
  idl_types_interface.ProgrammerInstance? _instance =
      idl_interface_constructor.ProgrammerConstructor.createInstance();

  Stream<int> commands(Stream<int> value) => _instance!.commands(value);
  @override
  dispose() {
    (_instance! as Disposable).dispose();
    _instance = null;
  }
}

class Point {
  const Point(this.x, this.y);

  final int x;

  final int y;

  @override
  bool operator ==(dynamic other) => other.x == x && other.y == y;
}

class Size {
  const Size(this.width, this.height);

  final double width;

  final double height;

  @override
  bool operator ==(dynamic other) =>
      other.width == width && other.height == height;
}