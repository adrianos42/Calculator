import 'package:calculator/calculator_icons.dart';
import 'package:desktop/desktop.dart';

import '../num_tile.dart';

import '../main.dart' as man;

enum NumPadAction {
  changeShift,
}

typedef NumPadActionCallback = void Function(NumPadAction);

class NumPad extends StatelessWidget {
  const NumPad({
    Key? key,
    required this.setAction,
  })  : super(key: key);

  final NumPadActionCallback setAction;

  @override
  Widget build(BuildContext context) {
    final Color eqColor = Theme.of(context).colorScheme.primary2.toColor();

    final Color symColor = Theme.of(context).colorScheme.background.toColor();
    final Color numColor = Color(0xFF0A0A0A);

    Widget result = Column(
      children: [
        Expanded(
          child: Row(
            //mainAxisAlignment: MainAxisAlignment.spaceEvenly,
            mainAxisSize: MainAxisSize.max,
            children: [
              Expanded(
                child: NumTile(child: Text('%'), color: symColor),
              ),
              Expanded(
                child: NumTile(child: Text('√'), color: symColor),
              ),
              Expanded(
                child: NumTile(child: Text('x²'), color: symColor),
              ),
              Expanded(
                child: NumTile(child: Text('¹/x'), color: symColor),
              ),
            ],
          ),
        ),
        Expanded(
          child: Row(
            mainAxisSize: MainAxisSize.max,
            children: [
              Expanded(
                child: NumTile(child: Text('CE'), color: symColor),
              ),
              Expanded(
                child: NumTile(child: Text('C'), color: symColor),
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
              Expanded(
                child: NumTile(
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
                child: NumTile(child: Text('7'), color: numColor),
              ),
              Expanded(
                child: NumTile(child: Text('8'), color: numColor),
              ),
              Expanded(
                child: NumTile(child: Text('9'), color: numColor),
              ),
              Expanded(
                child: NumTile(
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
                child: NumTile(child: Text('4'), color: numColor),
              ),
              Expanded(
                child: NumTile(child: Text('5'), color: numColor),
              ),
              Expanded(
                child: NumTile(child: Text('6'), color: numColor),
              ),
              Expanded(
                child: NumTile(
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
                child: NumTile(child: Text('1'), color: numColor),
              ),
              Expanded(
                child: NumTile(child: Text('2'), color: numColor),
              ),
              Expanded(
                child: NumTile(child: Text('3'), color: numColor),
              ),
              Expanded(
                child: NumTile(
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
                child: NumTile(child: Text('.'), color: numColor),
              ),
              Expanded(
                child: NumTile(child: Text('0'), color: numColor),
              ),
              Expanded(
                child: NumTile(child: Text('±'), color: numColor),
              ),
              Expanded(
                child: NumTile(
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
