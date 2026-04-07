import 'package:s2_sdk_dart/src/record.dart';
import 'package:s2_sdk_dart/src/rust/types.dart';

import 'rust/producer.dart' as inner;

final class S2Producer {
  final inner.S2Producer _appendSession;

  S2Producer(this._appendSession);

  Future<inner.RecordSubmitTicket> append(S2AppendRecord record) {
    return _appendSession.submit(
      record: AppendRecord(
        body: record.body,
        headers: record.headers ?? [],
        timestamp: record.timestamp,
      ),
    );
  }

  Future<void> close() {
    return _appendSession.close();
  }
}
