import 'idl_ffi.dart' as idl_ffi;
import 'idl_types_interface.dart' as idl_types_interface;

class ProgrammerConstructor {
  static idl_types_interface.ProgrammerInstance createInstance() =>
      idl_ffi.Programmer();
}