import 'package:desktop/desktop.dart';

import 'package:shared/programmer.dart';

class BinaryNumPad extends StatelessWidget {
  static const names = [
    ['0', '4', '8', '12'],
    ['16', '20', '24', '28'],
    ['32', '36', '40', '44'],
    ['48', '52', '56', '60'],
  ];

  bool isCurrentItemEnabled(int bitWidth, int row, int col) {
    return row * 0x10 + col * 0x4 < bitWidth;
  }

  Widget createZeroItem(BuildContext context, int bitwidth, int row, int col) {
    final textTheme = Theme.of(context).textTheme;
    final colorScheme = Theme.of(context).colorScheme;
    final color = textTheme.textMedium;
    final highlightColor = colorScheme.primary;

    final labelColor = isCurrentItemEnabled(bitwidth, row, col)
        ? textTheme.textMedium
        : textTheme.textLow;

    return Column(
      mainAxisSize: MainAxisSize.min,
      mainAxisAlignment: MainAxisAlignment.start,
      crossAxisAlignment: CrossAxisAlignment.end,
      children: [
        Row(
          mainAxisSize: MainAxisSize.min,
          children: List.generate(4, (index) {
            final int binIndex = row * 0x10 + col * 0x4 + index;

            return StreamBuilder(
              initialData: NumPad.binaryPosition[binIndex].value,
              stream: NumPad.binaryPosition[binIndex],
              builder: (context, snapshot) {
                final enabled = binIndex < bitwidth;
                final current = snapshot.data! as bool;
                return TextButton(
                  enabled && current ? '1' : '0',
                  onPressed: enabled
                      ? () => NumPad.binaryPosition[binIndex].add(!current)
                      : null,
                  color: current ? highlightColor : color,
                );
              },
            );
          }).reversed.toList(),
        ),
        Container(
          padding: EdgeInsets.only(right: 4.0),
          //alignment: Alignment.centerRight,
          child: Text(
            names[row][col],
            style: textTheme.caption.copyWith(
              color: labelColor,
            ),
          ),
        )
      ],
    );
  }

  Widget createZeroRow() {
    return StreamBuilder(
      initialData: ProgrammerMode.bitWidth.value,
      stream: ProgrammerMode.bitWidth,
      builder: (context, snapshot) {
        final int numBitWidth = kNumWidths[snapshot.data! as WordWidth]!;
        return Container(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.spaceAround,
            children: List.generate(4, (row) {
              return Container(
                padding: EdgeInsets.symmetric(vertical: 8.0),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.spaceEvenly,
                  children: List.generate(
                    4,
                    (col) => createZeroItem(context, numBitWidth, row, col),
                  ).reversed.toList(),
                ),
              );
            }).reversed.toList(),
          ),
        );
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    Widget result = createZeroRow();

    return ButtonTheme.merge(
      data: ButtonThemeData(
        bodyPadding: EdgeInsets.zero,
        highlightColor: Theme.of(context).colorScheme.primary,
        hoverColor: Theme.of(context).textTheme.textHigh,
        textStyle: Theme.of(context).textTheme.body2.copyWith(fontSize: 16.0),
      ),
      child: result,
    );
  }
}
