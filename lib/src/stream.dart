import 'package:s2_sdk_dart/src/append_session.dart';
import 'package:s2_sdk_dart/src/producer.dart';
import 'package:s2_sdk_dart/src/record.dart';
import 'package:s2_sdk_dart/src/rust/types.dart';

import 'rust/stream.dart' as inner;

export 'rust/types.dart' show ReadStart, ReadStop, ReadFrom, ReadLimits;

final class S2Stream {
  final inner.S2Stream _stream;

  S2Stream(this._stream);

  Future<AppendAck> append(
    List<S2AppendRecord> records, {
    int? matchSeqNum,
    String? fencingToken,
  }) {
    return _stream.append(
      input: AppendInput(
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

  Future<S2AppendSession> appendSession({
    int? maxUnackedBytes,
    int? maxUnackedBatches,
  }) async {
    final session = await _stream.appendSession(
      config: inner.AppendSessionConfig(
        maxUnackedBytes: maxUnackedBytes,
        maxUnackedBatches: maxUnackedBatches,
      ),
    );
    return S2AppendSession(session);
  }

  Future<StreamPosition> checkTail() {
    return _stream.checkTail();
  }

  Future<S2Producer> producer() async {
    final producer = await _stream.producer();
    return S2Producer(producer);
  }

  Future<ReadBatch> read({ReadStart? start, ReadStop? stop}) {
    return _stream.read(
      input: ReadInput(start: start, stop: stop),
    );
  }

  Future<Stream<SequencedRecord>> readSession({
    ReadStart? start,
    ReadStop? stop,
  }) {
    return _stream.readSession(
      input: ReadInput(start: start, stop: stop),
    );
  }
}
