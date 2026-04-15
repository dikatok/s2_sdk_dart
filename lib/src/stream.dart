import 'package:s2_sdk_dart/src/append_session.dart';
import 'package:s2_sdk_dart/src/producer.dart';
import 'package:s2_sdk_dart/src/record.dart';
import 'package:s2_sdk_dart/src/rust/types.dart';

import 'rust/stream.dart' as inner;

export 'rust/stream.dart' show BatchingConfig;
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
                  headers: e.getHeaderInBytes(),
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

  S2AppendSession appendSession({
    int? maxUnackedBytes,
    int? maxUnackedBatches,
  }) {
    final session = _stream.appendSession(
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

  S2Producer producer({
    int? maxUnackedBytes,
    inner.BatchingConfig? batching,
    String? fencingToken,
    int? matchSeqNum,
  }) {
    final producer = _stream.producer(
      config: inner.ProducerConfig(
        maxUnackedBytes: maxUnackedBytes,
        batching: batching,
        fencingToken: fencingToken,
        matchSeqNum: matchSeqNum,
      ),
    );
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
  }) async {
    return _stream.readSession(
      input: ReadInput(start: start, stop: stop),
    );
  }
}
