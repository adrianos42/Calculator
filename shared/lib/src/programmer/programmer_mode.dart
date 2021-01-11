import 'package:shared/programmer.dart';

import '../shared/caption_notification.dart';

import 'package:rxdart/rxdart.dart';

//import 'package:calc_manager/calc_manager.dart';

class ProgrammerMode {
  // static final bitWidthName =
  //    BehaviorSubject<String>.seeded(_kWordWidthNames[_currentWordWidth][0]);
  // static final bitWidthSize =
  //   BehaviorSubject<int>.seeded(_kNumWidths[_currentWordWidth]);

  static final bitShiftMode =
      BehaviorSubject<BitShiftMode>.seeded(BitShiftMode.arithmeticShift);
  static final bitWidth = BehaviorSubject<WordWidth>.seeded(WordWidth.qword);
  static final base = BehaviorSubject<NumBase>.seeded(NumBase.dec);

  static void sendCommand(int value) async {
   // await CalcManager.registerDisplayCallback((value) {
   //   print('finally from here $value');
   // });

   // await CalcManager.sendCommand(value);
  }
}

enum NumBase {
  hex,
  dec,
  oct,
  bin,
}

enum BitShiftMode {
  arithmeticShift,
  logicalShift,
  rotateCircularShift,
  rotateThroughCarryCircularShift,
}

enum WordWidth {
  byte,
  word,
  dword,
  qword,
}

const kNumWidths = {
  WordWidth.byte: 8,
  WordWidth.word: 16,
  WordWidth.dword: 32,
  WordWidth.qword: 64,
};

