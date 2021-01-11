import 'idl_types.dart' as idl_types;
import 'dart:typed_data';
import 'dart:collection';
import 'package:ffi_internal/ffi_internal.dart';

abstract class ProgrammerInstance {
  Stream<int> commands(Stream<int> value);
}