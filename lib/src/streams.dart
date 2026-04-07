import 'package:s2_sdk_dart/src/rust/basin.dart';

final class S2Streams {
  final S2Basin _basin;

  S2Streams(this._basin);

  Future<StreamInfo> create(String name, {StreamConfig? streamConfig}) {
    return _basin.createStream(
      input: CreateStreamInput(name: name, config: streamConfig),
    );
  }

  Future<void> delete(String name, {bool ignoreNotFound = false}) {
    return _basin.deleteStream(
      input: DeleteStreamInput(name: name, ignoreNotFound: ignoreNotFound),
    );
  }

  Future<StreamConfig> getConfig(String name) {
    return _basin.getStreamConfig(name: name);
  }

  Future<Stream<StreamInfo>> listAll({
    String? prefix,
    String? startAfter,
    bool? includeDeleted,
  }) {
    return _basin.listAllStreams(
      input: ListAllStreamsInput(
        prefix: prefix,
        startAfter: startAfter,
        includeDeleted: includeDeleted,
      ),
    );
  }

  Future<PageOfStreamInfo> list({
    String? prefix,
    String? startAfter,
    int? limit,
  }) {
    return _basin.listStreams(
      input: ListStreamsInput(
        prefix: prefix,
        startAfter: startAfter,
        limit: limit,
      ),
    );
  }

  Future<StreamConfig> reconfigure(
    String name, {
    required StreamConfig streamConfig,
  }) {
    return _basin.reconfigureStream(
      input: ReconfigureStreamInput(name: name, config: streamConfig),
    );
  }
}
