import 'package:calculator/calculator_icons.dart';
import 'package:desktop/desktop.dart';

import '../num_tile.dart';

import 'package:shared/programmer.dart';
import 'package:shared/shared.dart';

const _kBitShiftNames = {
  BitShiftMode.arithmeticShift: [
    'Arithmetic Shift',
    BitShiftMode.logicalShift,
  ],
  BitShiftMode.logicalShift: [
    'Logical Shift',
    BitShiftMode.rotateCircularShift,
  ],
  BitShiftMode.rotateCircularShift: [
    'Rotate Circular Shift',
    BitShiftMode.rotateThroughCarryCircularShift,
  ],
  BitShiftMode.rotateThroughCarryCircularShift: [
    'Rotate Through Carry Circular Shift',
    BitShiftMode.arithmeticShift,
  ],
};

class ProgrammerNumPad extends StatelessWidget {
  const ProgrammerNumPad({Key? key}) : super(key: key);

  void _changeShiftMode() {
    ProgrammerMode.sendCommand(24);
    
    var bitShiftMode = ProgrammerMode.bitShiftMode.valueWrapper!.value;

    if (CaptionNotification.hasCaption.valueWrapper!.value) {
      bitShiftMode = _kBitShiftNames[bitShiftMode]![1] as BitShiftMode;
      ProgrammerMode.bitShiftMode.add(bitShiftMode);
    }

    CaptionNotification.caption.add(_kBitShiftNames[bitShiftMode]![0] as String);
  }

  @override
  Widget build(BuildContext context) {
    final Color eqColor = Theme.of(context).colorScheme.primary2.toColor();

    final Color commandColor = Theme.of(context).colorScheme.background.toColor();
    final Color symColor = Theme.of(context).colorScheme.background.toColor();
    final Color numColor = Color(0xFF0A0A0A);

    Widget result = Column(
      children: [
        Expanded(
          child: Row(
            mainAxisSize: MainAxisSize.max,
            children: [
              Expanded(
                child: NumTile(
                  onPressed: () => NumPad.action.add(NumPadAction.and),
                  child: Text(
                    'AND',
                    style: Theme.of(context).textTheme.caption,
                  ),
                  color: commandColor,
                ),
              ),
              Expanded(
                child: NumTile(
                  onPressed: () => NumPad.action.add(NumPadAction.or),
                  child: Text(
                    'OR',
                    style: Theme.of(context).textTheme.caption,
                  ),
                  color: commandColor,
                ),
              ),
              Expanded(
                child: NumTile(
                  onPressed: () => NumPad.action.add(NumPadAction.not),
                  child: Text(
                    'NOT',
                    style: Theme.of(context).textTheme.caption,
                  ),
                  color: commandColor,
                ),
              ),
              Expanded(
                child: NumTile(
                  onPressed: () => NumPad.action.add(NumPadAction.nor),
                  child: Text(
                    'NOR',
                    style: Theme.of(context).textTheme.caption,
                  ),
                  color: commandColor,
                ),
              ),
              Expanded(
                child: NumTile(
                  onPressed: () => NumPad.action.add(NumPadAction.xor),
                  child: Text(
                    'XOR',
                    style: Theme.of(context).textTheme.caption,
                  ),
                  color: commandColor,
                ),
              ),
              Expanded(
                child: NumTile(
                  onPressed: () => NumPad.action.add(NumPadAction.nand),
                  child: Text(
                    'NAND',
                    style: Theme.of(context).textTheme.caption,
                  ),
                  color: commandColor,
                ),
              ),
            ],
          ),
        ),
        Expanded(
          child: Row(
            //mainAxisAlignment: MainAxisAlignment.spaceEvenly,
            mainAxisSize: MainAxisSize.max,
            children: [
              Expanded(
                child: NumTile(
                    onPressed: () => _changeShiftMode(),
                    child: Icon(
                      CalculatorIcons.shift_select,
                      color: Theme.of(context).textTheme.textMedium.toColor(),
                      size: 18.0,
                    ),
                    color: symColor),
              ),
              Expanded(
                child: NumTile(
                  onPressed: () => NumPad.action.add(NumPadAction.leftShift),
                  child: Text('<<'),
                  color: symColor,
                ),
              ),
              Expanded(
                child: NumTile(
                  onPressed: () =>
                      NumPad.action.add(NumPadAction.rightShift),
                  child: Text('>>'),
                  color: symColor,
                ),
              ),
              Expanded(
                child: NumTile(
                  child: Text('CE'),
                  color: symColor,
                ),
              ),
              Expanded(
                child: NumTile(
                    child: Icon(
                      Icons.backspace,
                      color: Theme.of(context).textTheme.textMedium.toColor(),
                      size: 16,
                    ),
                    color: symColor),
              ),
            ],
          ),
        ),
        Expanded(
          child: Row(
            mainAxisSize: MainAxisSize.max,
            children: [
              Expanded(
                child: NumTile(
                 onPressed: () => NumPad.action.add(NumPadAction.nA),
                  child: Text('A'),
                  color: numColor,
                ),
              ),
              Expanded(
                child: NumTile(
                  child: Text('('),
                  color: symColor,
                ),
              ),
              Expanded(
                child: NumTile(
                  child: Text(')'),
                  color: symColor,
                ),
              ),
              Expanded(
                child: NumTile(
                  child: Text('%'),
                  color: symColor,
                ),
              ),
              Expanded(
                child: NumTile(
                  onPressed: () => NumPad.action.add(NumPadAction.div),
                  child: DefaultTextStyle.merge(
                    style: TextStyle(fontSize: 24.0),
                    child: Text('+'),
                  ),
                  color: symColor,
                ),
              ),
            ],
          ),
        ),
        Expanded(
          child: Row(
            mainAxisSize: MainAxisSize.max,
            children: [
              Expanded(
                child: NumTile(
                  onPressed: () => NumPad.action.add(NumPadAction.nB),
                  child: Text('B'),
                  color: numColor,
                ),
              ),
              Expanded(
                child: NumTile(
                  onPressed: () => NumPad.action.add(NumPadAction.n7),
                  child: Text('7'),
                  color: numColor,
                ),
              ),
              Expanded(
                child: NumTile(
                  onPressed: () => NumPad.action.add(NumPadAction.n8),
                  child: Text('8'),
                  color: numColor,
                ),
              ),
              Expanded(
                child: NumTile(
                  onPressed: () => NumPad.action.add(NumPadAction.n9),
                  child: Text('9'),
                  color: numColor,
                ),
              ),
              Expanded(
                child: NumTile(
                    onPressed: () => NumPad.action.add(NumPadAction.mul),
                    child: DefaultTextStyle.merge(
                      style: TextStyle(fontSize: 24.0),
                      child: Text('×'),
                    ),
                    color: symColor),
              ),
            ],
          ),
        ),
        Expanded(
          child: Row(
            mainAxisSize: MainAxisSize.max,
            children: [
              Expanded(
                child: NumTile(
                  onPressed: () => NumPad.action.add(NumPadAction.nC),
                  child: Text('C'),
                  color: numColor,
                ),
              ),
              Expanded(
                child: NumTile(
                     onPressed: () => NumPad.action.add(NumPadAction.n4),
                  child: Text('4'),
                  color: numColor,
                ),
              ),
              Expanded(
                child: NumTile(
                  onPressed: () => NumPad.action.add(NumPadAction.n5),
                  child: Text('5'),
                  color: numColor,
                ),
              ),
              Expanded(
                child: NumTile(
                  onPressed: () => NumPad.action.add(NumPadAction.n6),
                  child: Text('6'),
                  color: numColor,
                ),
              ),
              Expanded(
                child: NumTile(
                    onPressed: () => NumPad.action.add(NumPadAction.sub),
                    child: DefaultTextStyle.merge(
                      style: TextStyle(fontSize: 24.0),
                      child: Text('-'),
                    ),
                    color: symColor),
              ),
            ],
          ),
        ),
        Expanded(
          child: Row(
            mainAxisSize: MainAxisSize.max,
            children: [
              Expanded(
                child: NumTile(
                  onPressed: () => NumPad.action.add(NumPadAction.nD),
                  child: Text('D'),
                  color: numColor,
                ),
              ),
              Expanded(
                child: NumTile(
                  onPressed: () => NumPad.action.add(NumPadAction.n1),
                  child: Text('1'),
                  color: numColor,
                ),
              ),
              Expanded(
                child: NumTile(
                  onPressed: () => NumPad.action.add(NumPadAction.n2),
                  child: Text('2'),
                  color: numColor,
                ),
              ),
              Expanded(
                child: NumTile(
                  onPressed: () => NumPad.action.add(NumPadAction.n3),
                  child: Text('3'),
                  color: numColor,
                ),
              ),
              Expanded(
                child: NumTile(
                    onPressed: () => NumPad.action.add(NumPadAction.add),
                    child: DefaultTextStyle.merge(
                      style: TextStyle(fontSize: 24.0),
                      child: Text('+'),
                    ),
                    color: symColor),
              ),
            ],
          ),
        ),
        Expanded(
          child: Row(
            mainAxisSize: MainAxisSize.max,
            children: [
              Expanded(
                child: NumTile(
                  onPressed: () => NumPad.action.add(NumPadAction.nE),
                  child: Text('E'),
                  color: numColor,
                ),
              ),
              Expanded(
                child: NumTile(
                  onPressed: () => NumPad.action.add(NumPadAction.nF),
                  child: Text('F'),
                  color: numColor,
                ),
              ),
              Expanded(
                child: NumTile(
                  onPressed: () => NumPad.action.add(NumPadAction.n0),
                  child: Text('0'),
                  color: numColor,
                ),
              ),
              Expanded(
                child: NumTile(
                  child: Text('±'),
                  color: numColor,
                ),
              ),
              Expanded(
                // TODO Change to font icons
                child: NumTile(
                    onPressed: () => NumPad.action.add(NumPadAction.equal),
                    child: DefaultTextStyle.merge(
                      style: TextStyle(fontSize: 24.0),
                      child: Text('='),
                    ),
                    color: eqColor),
              ),
            ],
          ),
        ),
      ],
    );

    return result;
  }
}
