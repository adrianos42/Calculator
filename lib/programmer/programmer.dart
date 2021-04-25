import 'package:desktop/desktop.dart';
import 'package:flutter/rendering.dart';
import 'package:shared/programmer.dart';

import 'pad.dart';
import 'binary_pad.dart';
import '../calculator_icons.dart';
import '../display.dart';

class _BaseDisplay extends StatelessWidget {
  _BaseDisplay({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final textTheme = Theme.of(context).textTheme;
    final selectedColor = Theme.of(context).colorScheme.primary;
    final color = textTheme.textLow;

    return ButtonTheme.merge(
      data: ButtonThemeData(
        trailingPadding: EdgeInsets.only(left: 12.0),
        bodyPadding: EdgeInsets.zero,
      ),
      child: StreamBuilder(
        initialData: ProgrammerMode.base.valueWrapper?.value,
        stream: ProgrammerMode.base,
        builder: (context, snapshot) {
          final NumBase base = snapshot.data! as NumBase;

          return Column(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              ButtonTheme.merge(
                data: ButtonThemeData(
                  hoverColor: base == NumBase.hex ? selectedColor : null,
                  highlightColor: selectedColor,
                ),
                child: Button(
                  body: Text('HEX'),
                  trailing: Text('216B'),
                  onPressed: () => ProgrammerMode.base.add(NumBase.hex),
                  color: base == NumBase.hex ? selectedColor : color,
                ),
              ),
              ButtonTheme.merge(
                data: ButtonThemeData(
                  hoverColor: base == NumBase.dec ? selectedColor : null,
                  highlightColor: selectedColor,
                ),
                child: Button(
                  body: Text('DEC'),
                  trailing: Text('8,555'),
                  onPressed: () => ProgrammerMode.base.add(NumBase.dec),
                  color: base == NumBase.dec ? selectedColor : color,
                ),
              ),
              ButtonTheme.merge(
                data: ButtonThemeData(
                  hoverColor: base == NumBase.oct ? selectedColor : null,
                  highlightColor: selectedColor,
                ),
                child: Button(
                  body: Text('OCT'),
                  trailing: Text('20 553'),
                  onPressed: () => ProgrammerMode.base.add(NumBase.oct),
                  color: base == NumBase.oct ? selectedColor : color,
                ),
              ),
              ButtonTheme.merge(
                data: ButtonThemeData(
                  hoverColor: base == NumBase.bin ? selectedColor : null,
                  highlightColor: selectedColor,
                ),
                child: Button(
                  body: Text('BIN'),
                  trailing: Text('0010 0001 0110 1011'),
                  onPressed: () => ProgrammerMode.base.add(NumBase.bin),
                  color: base == NumBase.bin ? selectedColor : color,
                ),
              ),
            ],
          );
        },
      ),
    );
  }
}

class ProgrammerCalculator extends StatelessWidget {
  //_Base currentBase = _Base.dec;

  @override
  Widget build(BuildContext context) {
    Widget result = Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Display(),
        Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          mainAxisAlignment: MainAxisAlignment.start,
          mainAxisSize: MainAxisSize.min,
          children: [
            _BaseDisplay(),
          ],
        ),
        Expanded(
          child: Tab(
            trailing: (context) => Row(
              children: [
                _WordMode(),
                TextButton(
                  'MS',
                  onPressed: () {},
                ),
                TextButton(
                  'M',
                  onPressed: () {},
                ),
              ],
            ),
            items: [
              TabItem(
                title: Icon(Icons.dialpad),
                builder: (context) => ProgrammerNumPad(),
              ),
              TabItem(
                title: Icon(CalculatorIcons.binary),
                builder: (context) => BinaryNumPad(),
              ),
            ],
          ),
        )
      ],
    );

    return result;
  }
}

class _WordMode extends StatelessWidget {
  _WordMode({
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      constraints: BoxConstraints.tightFor(width: 80.0),
      alignment: Alignment.center,
      child: StreamBuilder(
        stream: ProgrammerMode.bitWidth,
        initialData: ProgrammerMode.bitWidth.valueWrapper?.value,
        builder: (context, snapshot) {
          final bitWidth = snapshot.data! as WordWidth;

          return TextButton(
            _kWordWidthNames[bitWidth]![0] as String,
            onPressed: () => ProgrammerMode.bitWidth
                .add(_kWordWidthNames[bitWidth]![1] as WordWidth),
          );
        },
      ),
    );
  }
}

const _kWordWidthNames = {
  WordWidth.byte: ['BYTE', WordWidth.word],
  WordWidth.word: ['WORD', WordWidth.dword],
  WordWidth.dword: ['DWORD', WordWidth.qword],
  WordWidth.qword: ['QWORD', WordWidth.byte],
};
