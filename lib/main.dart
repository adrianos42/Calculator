import 'dart:async';
import 'dart:math';

import 'package:calculator/constants.dart';
import 'package:desktop/desktop.dart';

import 'standard/standard.dart';

import 'programmer/programmer.dart';
import 'package:shared/shared.dart';

void main() => runApp(MyApp());

class MyApp extends StatelessWidget {
  final ValueNotifier<String> topMenuNotifier = ValueNotifier<String>('');
  final ValueNotifier<bool> topMenuVisibleNotifier = ValueNotifier<bool>(false);

  @override
  Widget build(BuildContext context) {
    return DesktopApp(
      theme: ThemeData.dark(),
      home: Builder(builder: (context) {
        Widget topMenu = Column(
          mainAxisAlignment: MainAxisAlignment.start,
          mainAxisSize: MainAxisSize.min,
          children: [
            Container(
              padding: EdgeInsets.symmetric(vertical: 8.0),
              child: ButtonTheme.merge(
                data: ButtonThemeData(
                  color: Theme.of(context).textTheme.textLow,
                  highlightColor: Theme.of(context).colorScheme.primary,
                  iconThemeData: IconThemeData(size: 22.0),
                ),
                child: Row(
                  mainAxisAlignment: MainAxisAlignment.start,
                  children: [
                    ContextMenuButton(
                      Icons.menu,
                      value: 'prg',
                      itemBuilder: (context) => [
                        ContextMenuItem(
                          value: 'std',
                          child: Row(
                            children: [
                              Icon(Icons.dialpad),
                              Padding(
                                padding: EdgeInsets.only(left: 8.0),
                                child: Text('Standard'),
                              ),
                            ],
                          ),
                        ),
                        ContextMenuItem(
                          value: 'sci',
                          child: Row(
                            children: [
                              Icon(Icons.functions),
                              Padding(
                                padding: EdgeInsets.only(left: 8.0),
                                child: Text('Scientific'),
                              ),
                            ],
                          ),
                        ),
                        ContextMenuItem(
                          value: 'prg',
                          child: Row(
                            children: [
                              Icon(Icons.developer_board),
                              Padding(
                                padding: EdgeInsets.only(left: 8.0),
                                child: Text('Programmer'),
                              ),
                            ],
                          ),
                        ),
                        ContextMenuItem(
                          value: 'cnv',
                          child: Row(
                            children: [
                              Icon(Icons.refresh),
                              Padding(
                                padding: EdgeInsets.only(left: 8.0),
                                child: Text('Converter'),
                              ),
                            ],
                          ),
                        ),
                      ],
                    ),
                    // Icon(
                    //   Icons.developer_board,
                    //   color: Theme.of(context).textTheme.textLow,
                    //   size: 18.0,
                    // ),
                    // Text(
                    //   '',
                    //   style: Theme.of(context).textTheme.body2.copyWith(
                    //         color: Theme.of(context).textTheme.textMedium,
                    //       ),
                    // ),
                    Expanded(
                      child: _TopCaption(),
                    ),
                  ],
                ),
              ),
            ),
            // Expanded(
            //   flex: 1,
            //   child: Container(
            //     padding: EdgeInsets.symmetric(horizontal: 16.0, vertical: 4.0),
            //     alignment: Alignment.centerRight,
            //     child: Text('4 + 2 + 53 + 21 - 3',
            //         style: Theme.of(context).textTheme.body1.copyWith(color: Theme.of(context).textTheme.textLow)),
            //   ),
            // ),
            // Expanded(
            //   flex: 1,
            //   child: Container(
            //     padding: EdgeInsets.all(16.0),
            //     alignment: Alignment.bottomRight,
            //     child: LayoutBuilder(builder: (_, constraints) {
            //       final headerTheme = Theme.of(context).textTheme.hearder;
            //       return Text(
            //         '2',
            //         style: headerTheme.copyWith(
            //           fontSize: max(
            //             min(constraints.maxHeight, 82.0),
            //             headerTheme.fontSize,
            //           ),
            //         ),
            //       );
            //     }),
            //   ),
            // ),
          ],
        );

        Widget result = Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            topMenu,
            Expanded(
              flex: 1,
              child: ProgrammerCalculator(),
            ),
          ],
        );

        return Row(
          mainAxisSize: MainAxisSize.max,
          mainAxisAlignment: MainAxisAlignment.start,
          children: [
            Visibility(
              visible:
                  MediaQuery.of(context).orientation == Orientation.landscape,
              child: Expanded(
                flex: 2,
                child: HistoryPage(),
              ),
            ),
            Expanded(
              flex: 3,
              child: Container(
                constraints: BoxConstraints.tightFor(width: kDefaultPadWidth),
                child: result,
              ),
            ),
          ],
        );
      }),
      //home: createPage(context),
    );
  }
}

class HistoryPage extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Tab(
      items: [
        TabItem(
          title: Text('History'),
          builder: (context) => Center(child: Text('square root of 4 = 2')),
        ),
        TabItem(
          title: Text('Memory'),
          builder: (context) => Center(child: Text("2 + 43")),
        ),
      ],
    );
  }
}

class _TopCaption extends StatefulWidget {
  @override
  _TopCaptionState createState() => _TopCaptionState();
}

class _TopCaptionState extends State<_TopCaption>
    with TickerProviderStateMixin {
  late AnimationController fadeoutAnimationController;
  Timer? fadeoutTimer;

  @override
  void didChangeDependencies() {
    super.didChangeDependencies();
  }

  @override
  void initState() {
    super.initState();

    fadeoutAnimationController = AnimationController(
      vsync: this,
      duration: Duration(milliseconds: 100),
    );
  }

  @override
  void dispose() {
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final color = Theme.of(context).colorScheme.primary;

    return Builder(builder: (context) {
      return Container(
        alignment: Alignment.centerLeft,
        child: StreamBuilder(
          stream: CaptionNotification.caption,
          builder: (context, snapshot) {
            fadeoutTimer?.cancel();

            String text = '';

            if (snapshot.hasData) {
              CaptionNotification.hasCaption.add(true);

              fadeoutAnimationController.value = 1.0;

              text = snapshot.data! as String;

              fadeoutTimer = Timer(Duration(seconds: 2), () {
                CaptionNotification.hasCaption.add(false);

                fadeoutAnimationController.reverse();
                fadeoutTimer = null;
              });
            }

            return FadeTransition(
              opacity: fadeoutAnimationController,
              child: Text(
                text,
                overflow: TextOverflow.ellipsis,
                maxLines: 1,
                style: Theme.of(context)
                    .textTheme
                    .body2
                    .copyWith(color: color.toColor()),
              ),
            );
          },
        ),
      );
    });
  }
}
