import 'package:desktop/desktop.dart';

class Display extends StatefulWidget {
  @override
  _DisplayState createState() => _DisplayState();
}

class _DisplayState extends State<Display> {
  @override
  Widget build(BuildContext context) {
    final textTheme = Theme.of(context).textTheme;
    final resultTextTheme = textTheme.title;
    final topTextTheme = textTheme.body1.copyWith(
      color: textTheme.textMedium,
    );

    return Container(
      padding: EdgeInsets.symmetric(horizontal: 8.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.end,
        children: [
          Padding(
            padding: EdgeInsets.symmetric(vertical: 2.0),
            child: Text(
              '8555 =',
              style: topTextTheme,
            ),
          ),
          Padding(
            padding: EdgeInsets.symmetric(vertical: 2.0),
            child: Text(
              '8,555',
              style: resultTextTheme,
            ),
          ),
        ],
      ),
    );
  }
}