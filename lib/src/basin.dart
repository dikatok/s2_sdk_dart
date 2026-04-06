import 'package:s2_sdk_dart/src/rust/stream.dart';

import 'rust/basin.dart' as inner;

final class S2Basin {
  final inner.S2Basin _basin;

  S2Basin(this._basin);

  S2Stream stream(String name) {
    return _basin.stream(name: name);
  }
}
