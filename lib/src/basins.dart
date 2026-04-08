import 'package:s2_sdk_dart/src/rust/basin.dart';
import 'package:s2_sdk_dart/src/rust/client.dart';

export 'rust/basin.dart'
    show
        StreamConfig,
        TimestampingMode,
        RetentionPolicy,
        TimestampingConfig,
        DeleteOnEmptyConfig,
        StorageClass;
export 'rust/client.dart' show BasinScope;

final class S2Basins {
  final S2Client _client;

  S2Basins(this._client);

  Future<BasinInfo> create(
    String name, {
    StreamConfig? defaultStreamConfig,
    bool createStreamOnAppend = false,
    bool createStreamOnRead = false,
    BasinScope? scope,
  }) {
    return _client.createBasin(
      input: CreateBasinInput(
        name: name,
        config: BasinConfig(
          defaultStreamConfig: defaultStreamConfig,
          createStreamOnAppend: createStreamOnAppend,
          createStreamOnRead: createStreamOnRead,
        ),
        scope: scope,
      ),
    );
  }

  Future<void> delete(String name, {bool ignoreNotFound = false}) {
    return _client.deleteBasin(
      input: DeleteBasinInput(name: name, ignoreNotFound: ignoreNotFound),
    );
  }

  Future<BasinConfig> getConfig(String name) {
    return _client.getBasinConfig(name: name);
  }

  Future<Stream<BasinInfo>> listAll({
    String? prefix,
    String? startAfter,
    bool? includeDeleted,
  }) async {
    return _client.listAllBasins(
      input: ListAllBasinsInput(
        prefix: prefix,
        startAfter: startAfter,
        includeDeleted: includeDeleted,
      ),
    );
  }

  Future<PageOfBasinInfo> list({
    String? prefix,
    String? startAfter,
    int? limit,
  }) {
    return _client.listBasins(
      input: ListBasinsInput(
        prefix: prefix,
        startAfter: startAfter,
        limit: limit,
      ),
    );
  }

  Future<BasinConfig> reconfigure(
    String name, {
    StreamConfig? defaultStreamConfig,
    bool createStreamOnAppend = false,
    bool createStreamOnRead = false,
  }) {
    return _client.reconfigureBasin(
      input: ReconfigureBasinInput(
        name: name,
        config: BasinConfig(
          defaultStreamConfig: defaultStreamConfig,
          createStreamOnAppend: createStreamOnAppend,
          createStreamOnRead: createStreamOnRead,
        ),
      ),
    );
  }
}
