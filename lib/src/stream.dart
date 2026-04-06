import 'package:s2_sdk_dart/s2_sdk_dart.dart';

import 'rust/stream.dart' as inner;

final class S2Stream {
  final inner.S2Stream _stream;

  S2Stream(this._stream);

  Future<AppendAck> append(AppendInput input) {
    return _stream.append(input: input);
  }

  Future<S2AppendSession> appendSession(AppendSessionConfig input) async {
    final session = await _stream.appendSession(config: input);
    return S2AppendSession(session);
  }

  Future<StreamPosition> checkTail() {
    return _stream.checkTail();
  }

  Future<S2Producer> producer() async {
    final producer = await _stream.producer();
    return S2Producer(producer);
  }

  Future<ReadBatch> read(ReadInput input) {
    return _stream.read(input: input);
  }

  Future<Stream<SequencedRecord>> readSession(ReadInput input) {
    return _stream.readSession(input: input);
  }
}
