import 'package:s2_sdk_dart/src/record.dart';
import 'package:s2_sdk_dart/src/rust/types.dart';

import 'rust/append_session.dart' as inner;

final class S2AppendSession {
  final inner.S2AppendSession _appendSession;

  S2AppendSession(this._appendSession);

  Future<inner.BatchSubmitTicket> append(
    List<S2AppendRecord> records, {
    int? matchSeqNum,
    String? fencingToken,
  }) {
    return _appendSession.submit(
      record: AppendInput(
        records: AppendRecordBatch(
          records: records
              .map(
                (e) => AppendRecord(
                  body: e.body,
                  headers: e.headers ?? [],
                  timestamp: e.timestamp,
                ),
              )
              .toList(),
        ),
        matchSeqNum: matchSeqNum,
        fencingToken: fencingToken,
      ),
    );
  }

  Future<void> close() {
    return _appendSession.close();
  }
}
