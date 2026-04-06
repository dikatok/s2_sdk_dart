import 'package:s2_sdk_dart/src/rust/basin.dart';

final class S2Streams {
  final S2Basin _basin;

  S2Streams(this._basin);

  Future<StreamInfo> create(CreateStreamInput input) {
    return _basin.createStream(input: input);
  }

  Future<void> delete(DeleteStreamInput input) {
    return _basin.deleteStream(input: input);
  }

  Future<StreamConfig> getConfig(String name) {
    return _basin.getStreamConfig(name: name);
  }

  Future<Stream<StreamInfo>> listAll(ListAllStreamsInput input) {
    return _basin.listAllStreams(input: input);
  }

  Future<PageOfStreamInfo> list(ListStreamsInput input) {
    return _basin.listStreams(input: input);
  }

  Future<StreamConfig> reconfigure(ReconfigureStreamInput input) {
    return _basin.reconfigureStream(input: input);
  }
}
