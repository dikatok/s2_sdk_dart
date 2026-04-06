import 'package:s2_sdk_dart/src/rust/types.dart';

import 'rust/producer.dart' as inner;

final class S2Producer {
  final inner.S2Producer _appendSession;

  S2Producer(this._appendSession);

  Future<inner.RecordSubmitTicket> append(AppendRecord input) {
    return _appendSession.submit(record: input);
  }

  Future<void> close() {
    return _appendSession.close();
  }
}
