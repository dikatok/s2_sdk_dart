import 'package:s2_sdk_dart/src/rust/client.dart';

final class S2Basins {
  final S2Client _client;

  S2Basins(this._client);

  Future<BasinInfo> create(CreateBasinInput input) {
    return _client.createBasin(input: input);
  }

  Future<void> delete(DeleteBasinInput input) {
    return _client.deleteBasin(input: input);
  }

  Future<BasinConfig> getConfig(String name) {
    return _client.getBasinConfig(name: name);
  }

  Future<Stream<BasinInfo>> listAll(ListAllBasinsInput input) {
    return _client.listAllBasins(input: input);
  }

  Future<PageOfBasinInfo> list(ListBasinsInput input) {
    return _client.listBasins(input: input);
  }

  Future<BasinConfig> reconfigure(ReconfigureBasinInput input) {
    return _client.reconfigureBasin(input: input);
  }
}
