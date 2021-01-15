import 'idl_types_interface.dart' as idl_types_interface;
import 'idl_types.dart' as idl_types;
import 'dart:ffi';
import 'dart:async';
import 'dart:typed_data';
import 'dart:isolate';
import 'dart:collection';
import 'package:ffi/ffi.dart';
import 'package:ffi_internal/ffi_internal.dart';

final DynamicLibrary _$kLib =
    openLibrary('calc_manager', '../calc_manager_impl/target/release/');

class Programmer implements idl_types_interface.ProgrammerInstance, Disposable {
  ReceivePort? _$toMainSendCommandsPort;

  ReceivePort? _$toMainCommandsPort;

  SendPort? _$toIsolatePort;

  ReceivePort? _$toMainPort;

  Isolate? _$isolate;

  static final _$InstanceCreateProgrammerFunc _$instanceCreate =
      _$kLib.lookupFunction<_$InstanceCreateProgrammerNative,
          _$InstanceCreateProgrammerFunc>('create_programmer');

  static final _$InstanceDisposeProgrammerFunc _$instanceDispose =
      _$kLib.lookupFunction<_$InstanceDisposeProgrammerNative,
          _$InstanceDisposeProgrammerFunc>('dispose_programmer');

  static final _$MethodProgrammerCommandsFunc _$methodCommands =
      _$kLib.lookupFunction<_$MethodProgrammerCommandsNative,
          _$MethodProgrammerCommandsFunc>('method_programmer_commands');

  static final _$DisposeProgrammerCommandsFunc _$disposeStreamCommands =
      _$kLib.lookupFunction<_$DisposeProgrammerCommandsNative,
              _$DisposeProgrammerCommandsFunc>(
          'dispose_stream_programmer_commands');

  final _$streamsCommands = HashMap<int, List<dynamic>>();

  int _$handleSendCommands = 1;

  static final _$DisposeProgrammerCommandsFunc _$disposeStreamSenderCommands =
      _$kLib.lookupFunction<_$DisposeProgrammerCommandsNative,
              _$DisposeProgrammerCommandsFunc>(
          'dispose_stream_sender_programmer_commands');

  static final _$StreamProgrammerCommandsFunc _$streamSenderCommands =
      _$kLib.lookupFunction<_$StreamProgrammerCommandsNative,
          _$StreamProgrammerCommandsFunc>('stream_sender_programmer_commands');

  final _$streamControllersCommands = HashMap<int, StreamController<int>>();

  int _$handleCommands = 1;

  static final _$StreamProgrammerCommandsFunc _$streamCommands =
      _$kLib.lookupFunction<_$StreamProgrammerCommandsNative,
          _$StreamProgrammerCommandsFunc>('stream_programmer_commands');

  @override
  Stream<int> commands(Stream<int> value) {
    while (_$streamsCommands.containsKey(_$handleSendCommands)) {
      _$handleSendCommands += 1;
    }
    _$streamsCommands[_$handleSendCommands] = [value, null];
    while (_$streamControllersCommands.containsKey(_$handleCommands)) {
      _$handleCommands += 1;
    }
    final $handle = _$handleCommands;
    final $controller = StreamController<int>(onListen: () {
      _$toIsolate().then(($value) => $value.send([
            'commands',
            [_$toMainCommandsPort!.sendPort.nativePort, $handle],
            [
              _$toMainSendCommandsPort!.sendPort.nativePort,
              _$handleSendCommands
            ]
          ]));
    }, onCancel: () {
      _$toIsolatePort?.send([
        StreamReceiverState.close,
        $handle,
        'commands',
        _$toMainCommandsPort!.sendPort.nativePort,
        _$toMainCommandsPort!.sendPort,
      ]);
    }, onPause: () {
      _$toIsolatePort?.send([
        StreamReceiverState.pause,
        $handle,
        'commands',
        _$toMainCommandsPort!.sendPort.nativePort,
        _$toMainCommandsPort!.sendPort,
      ]);
    }, onResume: () {
      _$toIsolatePort?.send([
        StreamReceiverState.resume,
        $handle,
        'commands',
        _$toMainCommandsPort!.sendPort.nativePort,
        _$toMainCommandsPort!.sendPort,
      ]);
    });
    _$streamControllersCommands[$handle] = $controller;
    return $controller.stream;
  }

  Future<SendPort> _$toIsolate() async {
    _$toIsolatePort ??= await _$setIsolate();
    return _$toIsolatePort!;
  }

  Future<SendPort> _$setIsolate() async {
    final $completer = Completer<SendPort>();
    _$toMainPort = ReceivePort();
    _$toMainPort!.listen(($data) {
      if ($data is SendPort) {
        $completer.complete($data);
      }
    });
    _$toMainSendCommandsPort = ReceivePort();
    _$toMainSendCommandsPort!.listen(($data) {
      if ($data is int) {
        _$toIsolatePort?.send([
          StreamSenderState.request,
          $data,
          'commands',
          _$toMainSendCommandsPort!.sendPort.nativePort,
          _$toMainSendCommandsPort!.sendPort,
          0,
        ]);
      } else if ($data is List<dynamic>) {
        final $handleValue = $data[0] as int;
        if (!_$streamsCommands.containsKey($handleValue)) return;
        final $state = StreamReceiverState.values[$data[1] as int];
        switch ($state) {
          case StreamReceiverState.start:
            final $stream = _$streamsCommands[$handleValue]![0] as Stream<int>;
            final $streamSubscription = $stream.listen(($streamData) {
              _$toIsolatePort?.send([
                StreamSenderState.value,
                $handleValue,
                'commands',
                _$toMainSendCommandsPort!.sendPort.nativePort,
                _$toMainSendCommandsPort!.sendPort,
                $streamData,
              ]);
            });
            $streamSubscription.onDone(() {
              _$streamsCommands.remove($handleValue);
              _$toIsolatePort?.send([
                StreamSenderState.done,
                $handleValue,
                'commands',
                _$toMainSendCommandsPort!.sendPort.nativePort,
                _$toMainSendCommandsPort!.sendPort,
                0,
              ]);
            });
            _$streamsCommands[$handleValue]![1] = $streamSubscription;
            break;
          case StreamReceiverState.pause:
            final $streamSubscription =
                _$streamsCommands[$handleValue]![1]! as StreamSubscription<int>;
            $streamSubscription.pause();
            break;
          case StreamReceiverState.resume:
            final $streamSubscription =
                _$streamsCommands[$handleValue]![1]! as StreamSubscription<int>;
            $streamSubscription.resume();
            break;
          case StreamReceiverState.close:
            final $streams = _$streamsCommands.remove($handleValue)!;
            final $streamSubscription = $streams[1]! as StreamSubscription<int>;
            $streamSubscription.cancel();
            break;
          case StreamReceiverState.ok:
            break;
          default:
            throw ArgumentError('Invalid stream state `${$state}`');
        }
      }
    });
    _$toMainCommandsPort = ReceivePort();
    _$toMainCommandsPort!.listen(($data) {
      if ($data is int) {
        _$toIsolatePort?.send([
          StreamReceiverState.request,
          $data,
          'commands',
          _$toMainCommandsPort!.sendPort.nativePort,
          _$toMainCommandsPort!.sendPort,
        ]);
      } else if ($data is List<dynamic>) {
        final $handleValue = $data[0] as int;
        if (!_$streamControllersCommands.containsKey($handleValue)) return;
        final $state = StreamSenderState.values[$data[1] as int];
        switch ($state) {
          case StreamSenderState.value:
            _$streamControllersCommands[$handleValue]!.add($data[2] as int);
            break;
          case StreamSenderState.done:
            final $controller =
                _$streamControllersCommands.remove($handleValue)!;
            $controller.close();
            break;
          case StreamSenderState.waiting:
            break;
          case StreamSenderState.ok:
            break;
          default:
            throw ArgumentError('Invalid stream state `${$state}`');
        }
      }
    });
    _$isolate = await Isolate.spawn(_$runIsolate, _$toMainPort!.sendPort);
    return $completer.future;
  }

  static void _$runIsolate(SendPort $sendPort) async {
    final $receivePort = ReceivePort();
    final $instance = _$create();
    $sendPort.send($receivePort.sendPort);
    await for (var $data in $receivePort) {
      if ($data is List<dynamic>) {
        if ($data[0] is String) {
          switch ($data[0]) {
            case 'commands':
              ($wakePortSend, $wakeObjectSend, $wakePort, $wakeObject) {
                final $fValueValue = allocate<AbiStream>();
                final $streamDataValue = allocate<Int64>();
                final $streamValue = allocate<AbiStream>();
                try {
                  $streamDataValue.value = -1;
                  $fValueValue.ref.wakeHandle = $wakePortSend;
                  $fValueValue.ref.wakeObject =
                      DartCObjectInt.newObject($wakeObjectSend).cast();
                  $fValueValue.ref.wakeCallback = NativeApi.postCObject.cast();
                  $fValueValue.ref.state = AbiStreamSenderState.waiting;
                  $streamValue.ref.data = $streamDataValue.cast();
                  $streamValue.ref.wakeHandle = $wakePort;
                  $streamValue.ref.wakeObject =
                      DartCObjectInt.newObject($wakeObject).cast();
                  $streamValue.ref.wakeCallback = NativeApi.postCObject.cast();
                  $streamValue.ref.state = AbiStreamReceiverState.start;
                  AbiInternalError.handleError(
                      _$methodCommands($instance, $fValueValue, $streamValue),
                      'Programmer.commands');
                } finally {
                  free($streamDataValue);
                  free($fValueValue);
                  free($streamValue);
                }
              }($data[2][0] as int, $data[2][1] as int, $data[1][0] as int,
                  $data[1][1] as int);
              break;
            default:
              throw ArgumentError('Invalid port args');
          }
        } else if ($data[0] is StreamReceiverState) {
          switch ($data[2] as String) {
            case 'commands':
              (int $streamState, int $wakeObject, int $wakePort,
                      SendPort $sendPort) {
                final $streamValue = allocate<AbiStream>();
                final $fValue = allocate<Pointer<AbiStream>>();
                try {
                  $streamValue.ref.wakeHandle = $wakePort;
                  $streamValue.ref.wakeObject =
                      DartCObjectInt.newObject($wakeObject).cast();
                  $streamValue.ref.wakeCallback = NativeApi.postCObject.cast();
                  $streamValue.ref.state = $streamState;
                  AbiInternalError.handleError(
                      _$streamCommands($instance, $streamValue, $fValue),
                      'Programmer.commands');
                  try {
                    final $stream = $fValue.value;
                    switch ($stream.ref.state!) {
                      case AbiStreamSenderState.ok:
                        break;
                      case AbiStreamSenderState.done:
                        $sendPort.send([$wakeObject, $stream.ref.state!, 0]);
                        break;
                      case AbiStreamSenderState.value:
                        final $result = $stream.ref.data!.cast<Int64>().value;
                        $sendPort
                            .send([$wakeObject, $stream.ref.state!, $result]);
                        break;
                      default:
                        throw ArgumentError();
                    }
                  } finally {
                    AbiInternalError.handleError(
                        _$disposeStreamCommands($instance, $fValue.value),
                        'Programmer.commands');
                  }
                } finally {
                  free($streamValue);
                  free($fValue);
                }
              }(($data[0] as StreamReceiverState).index, $data[1] as int,
                  $data[3] as int, $data[4] as SendPort);
              break;
            default:
              throw ArgumentError('Invalid port args');
          }
        } else if ($data[0] is StreamSenderState) {
          switch ($data[2] as String) {
            case 'commands':
              (int $streamState, int $wakeObject, int $wakePort,
                      SendPort $sendPort, int $streamData) {
                final $streamValue = allocate<AbiStream>();
                final $fValue = allocate<Pointer<AbiStream>>();
                try {
                  switch ($streamState) {
                    case AbiStreamSenderState.value:
                      $streamValue.ref.data = ((int $value) {
                        final $result = allocate<Int64>();
                        $result.value = $value;
                        return $result;
                      }($streamData))
                          .cast();
                      break;
                    default:
                      break;
                  }
                  $streamValue.ref.wakeHandle = $wakePort;
                  $streamValue.ref.wakeObject =
                      DartCObjectInt.newObject($wakeObject).cast();
                  $streamValue.ref.wakeCallback = NativeApi.postCObject.cast();
                  $streamValue.ref.state = $streamState;
                  AbiInternalError.handleError(
                      _$streamSenderCommands($instance, $streamValue, $fValue),
                      'Programmer.commands');
                  try {
                    final $stream = $fValue.value;
                    switch ($stream.ref.state!) {
                      case AbiStreamReceiverState.ok:
                        break;
                      case AbiStreamReceiverState.close:
                      case AbiStreamReceiverState.start:
                      case AbiStreamReceiverState.pause:
                      case AbiStreamReceiverState.resume:
                        $sendPort.send([$wakeObject, $stream.ref.state!]);
                        break;
                      default:
                        throw ArgumentError();
                    }
                  } finally {
                    AbiInternalError.handleError(
                        _$disposeStreamSenderCommands($instance, $fValue.value),
                        'Programmer.commands');
                  }
                } finally {
                  free($streamValue);
                  free($fValue);
                }
              }(($data[0] as StreamSenderState).index, $data[1] as int,
                  $data[3] as int, $data[4] as SendPort, $data[5] as int);
              break;
            default:
              throw ArgumentError('Invalid port args');
          }
        }
      } else {
        break;
      }
    }
    AbiInternalError.handleError(_$instanceDispose($instance), 'Programmer');
  }

  static Pointer<_Programmer> _$create() {
    final $result = allocate<Pointer<_Programmer>>();
    try {
      AbiInternalError.handleError(_$instanceCreate($result), 'Programmer');
      return $result.value;
    } finally {
      free($result);
    }
  }

  @override
  void dispose() {
    _$toIsolatePort?.send(false);
    _$toMainPort?.close();
    _$isolate?.kill();
    _$toMainSendCommandsPort?.close();
    _$toMainSendCommandsPort = null;
    _$toMainCommandsPort?.close();
    _$toMainCommandsPort = null;
    _$toIsolatePort = null;
    _$toMainPort = null;
    _$isolate = null;
  }
}

class _Programmer extends Struct {}

class Point extends Struct {
  @Int64()
  int? x;

  @Int64()
  int? y;

  static idl_types.Point _$asValue(Pointer<Point> $value) =>
      idl_types.Point($value.ref.x!, $value.ref.y!);
  static void _$fromWithPtr(idl_types.Point $value, Pointer<Point> $result) {
    $result.ref.x = $value.x;
    $result.ref.y = $value.y;
  }

  static Pointer<Point> _$from(idl_types.Point $value) {
    final $result = allocate<Point>();
    _$fromWithPtr($value, $result);
    return $result;
  }

  static _$dispose(Pointer<Point> $value) {
    _$disposeWithPtr($value);
    free($value);
  }

  static _$disposeWithPtr(Pointer<Point> $value) {}
}

class Size extends Struct {
  @Double()
  double? width;

  @Double()
  double? height;

  static idl_types.Size _$asValue(Pointer<Size> $value) =>
      idl_types.Size($value.ref.width!, $value.ref.height!);
  static void _$fromWithPtr(idl_types.Size $value, Pointer<Size> $result) {
    $result.ref.width = $value.width;
    $result.ref.height = $value.height;
  }

  static Pointer<Size> _$from(idl_types.Size $value) {
    final $result = allocate<Size>();
    _$fromWithPtr($value, $result);
    return $result;
  }

  static _$dispose(Pointer<Size> $value) {
    _$disposeWithPtr($value);
    free($value);
  }

  static _$disposeWithPtr(Pointer<Size> $value) {}
}

typedef _$InstanceCreateProgrammerNative = Int64 Function(
    Pointer<Pointer<_Programmer>>);
typedef _$InstanceCreateProgrammerFunc = int Function(
    Pointer<Pointer<_Programmer>>);
typedef _$InstanceDisposeProgrammerNative = Int64 Function(
    Pointer<_Programmer>);
typedef _$InstanceDisposeProgrammerFunc = int Function(Pointer<_Programmer>);
typedef _$DisposeProgrammerCommandsNative = Int64 Function(
    Pointer<_Programmer>, Pointer<AbiStream>);
typedef _$DisposeProgrammerCommandsFunc = int Function(
    Pointer<_Programmer>, Pointer<AbiStream>);
typedef _$MethodProgrammerCommandsNative = Int64 Function(
    Pointer<_Programmer>, Pointer<AbiStream>, Pointer<AbiStream>);
typedef _$MethodProgrammerCommandsFunc = int Function(
    Pointer<_Programmer>, Pointer<AbiStream>, Pointer<AbiStream>);
typedef _$StreamProgrammerCommandsNative = Int64 Function(
    Pointer<_Programmer>, Pointer<AbiStream>, Pointer<Pointer<AbiStream>>);
typedef _$StreamProgrammerCommandsFunc = int Function(
    Pointer<_Programmer>, Pointer<AbiStream>, Pointer<Pointer<AbiStream>>);