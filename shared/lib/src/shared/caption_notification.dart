import 'package:rxdart/rxdart.dart';

class CaptionNotification {
  static final caption = BehaviorSubject<String>.seeded('');
  static final hasCaption = BehaviorSubject<bool>.seeded(false);
}
