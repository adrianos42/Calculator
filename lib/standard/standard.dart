import 'package:desktop/desktop.dart';
import 'package:flutter/rendering.dart';

import 'num_pad.dart';
import '../display.dart';

class StandardCalculator extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    final Color numColor = Theme.of(context).colorScheme.overlay2;
    final Color symColor = Theme.of(context).colorScheme.overlay1;

    void setAction(NumPadAction action) {}

    Widget result = Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Expanded(
          flex: 1,
          child: Display(),
        ),
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceAround,
          mainAxisSize: MainAxisSize.max,
          crossAxisAlignment: CrossAxisAlignment.center,
          children: [
            TextButton('MC', onPressed: () {}),
            TextButton('MR', onPressed: () {}),
            TextButton('M+', onPressed: null),
            TextButton('M-', onPressed: null),
            TextButton('MS', onPressed: () {}),
            TextButton('M', onPressed: () {}),
          ],
        ),
        Expanded(
          flex: 1,
          child: NumPad(setAction: setAction),
        )
      ],
    );

    return result;
  }
}
