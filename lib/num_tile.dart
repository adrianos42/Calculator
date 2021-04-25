import 'package:desktop/desktop.dart';
import 'package:flutter/rendering.dart';

class NumTile extends StatefulWidget {
  const NumTile({
    Key? key,
    required this.child,
    required this.color,
    this.onPressed,
  }) : super(key: key);

  final Widget child;

  final Color color;

  final VoidCallback? onPressed;

  @override
  _NumTileState createState() => _NumTileState();
}

class _NumTileState extends State<NumTile> with ComponentStateMixin {
  void _handleHoverEntered() {
    if (!hovered) setState(() => hovered = true);
  }

  void _handleHoverExited() {
    if (hovered) setState(() => hovered = false);
  }

  void _handleTapUp(TapUpDetails event) {
    if (pressed) setState(() => pressed = false);
  }

  void _handleTapDown(TapDownDetails event) {
    if (!pressed) setState(() => pressed = true);
  }

  void _handleTapCancel() {
    if (pressed) setState(() => pressed = false);
  }

  bool _globalPointerDown = false;

  void _mouseRoute(event) => _globalPointerDown = event.down;

  @override
  void initState() {
    super.initState();

    WidgetsBinding.instance!.pointerRouter.addGlobalRoute(_mouseRoute);
  }

  @override
  void dispose() {
    WidgetsBinding.instance!.pointerRouter.removeGlobalRoute(_mouseRoute);
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final ColorScheme colorScheme = Theme.of(context).colorScheme;
    final TextTheme textTheme = Theme.of(context).textTheme;

    final Color enabledForeground = widget.color;
    final Color pressedForeground = colorScheme.primary.toColor();
    final Color hoveredForeground = colorScheme.shade6.toColor();

    final Color color = pressed
        ? pressedForeground
        : hovered
            ? hoveredForeground
            : enabledForeground;

    Widget result = DefaultTextStyle.merge(
      style: Theme.of(context).textTheme.body1.copyWith(fontSize: 18.0),
      child: widget.child,
    );

    result = Container(
      alignment: Alignment.center,
      color: color,
      child: result,
    );

    result = MouseRegion(
      cursor: SystemMouseCursors.click,
      onEnter: (_) => _handleHoverEntered(),
      onExit: (_) => _handleHoverExited(),
      child: GestureDetector(
        onTap: widget.onPressed,
        onTapUp: _handleTapUp,
        onTapDown: _handleTapDown,
        onTapCancel: _handleTapCancel,
        child: result,
      ),
    );

    return result;
  }
}
