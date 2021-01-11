import 'package:shared/programmer.dart';

import 'programmer_mode.dart';

import 'package:rxdart/rxdart.dart';

const kMaxBitWidth = 64;

enum NumPadAction {
  changeShift,
  leftShift,
  rightShift,
  and,
  or,
  not,
  nor,
  xor,
  nand,
  add,
  sub,
  div,
  mul,
  mod,
  root,
  pwr,
  equal,
  backspace,
  n0,
  n1,
  n2,
  n3,
  n4,
  n5,
  n6,
  n7,
  n8,
  n9,
  nA,
  nB,
  nC,
  nD,
  nE,
  nF,
}

class NumPad {
  static final action = BehaviorSubject<NumPadAction>()
    ..listen((value) {
      switch (value) {
        case NumPadAction.n0:
          break;
        case NumPadAction.n1:
          break;
        case NumPadAction.n2:
          break;
        case NumPadAction.n3:
          break;
        case NumPadAction.n4:
          break;
        case NumPadAction.n5:
          break;
        case NumPadAction.n6:
          break;
        case NumPadAction.n7:
          break;
        case NumPadAction.n8:
          break;
        case NumPadAction.n9:
          break;
        case NumPadAction.nA:
          break;
        case NumPadAction.nB:
          break;
        case NumPadAction.nC:
          break;
        case NumPadAction.nD:
          break;
        case NumPadAction.nE:
          break;
        case NumPadAction.nF:
          break;
        default:
          break;
      }
    });

  static final binaryPosition = List.generate(
    kMaxBitWidth,
    (index) => BehaviorSubject<bool>.seeded(false)
      ..listen(
        (value) {
        },
      ),
  );
}
