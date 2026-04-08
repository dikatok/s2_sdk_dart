import 'package:s2_sdk_dart/src/stream.dart';
import 'package:s2_sdk_dart/src/streams.dart';

import 'rust/basin.dart' as inner;

final class S2Basin {
  final inner.S2Basin _basin;

  S2Basin(this._basin);

  S2Stream stream(String name) {
    return S2Stream(_basin.stream(name: name));
  }

  S2Streams get streams {
    return S2Streams(_basin);
  }
}
