import 'package:s2_sdk_dart/src/access_tokens.dart';
import 'package:s2_sdk_dart/src/basin.dart';
import 'package:s2_sdk_dart/src/basins.dart';
import 'package:s2_sdk_dart/src/rust/frb_generated.dart';
import 'package:s2_sdk_dart/src/rust/types.dart';

import 'rust/client.dart' as inner;

export 'rust/types.dart' show Compression, AppendRetryPolicy, RetryConfig;

final class S2 {
  final inner.S2Client _client;

  S2._(this._client);

  static Future<S2> create(
    String accessToken, {
    String? endpoint,
    int? connectionTimeoutMillis,
    int? requestTimeoutMillis,
    Compression? compression,
    RetryConfig? retryConfig,
  }) async {
    await RustLib.init();
    return S2._(
      await inner.S2Client.newInstance(
        config: ClientConfig(
          accessToken: accessToken,
          endpoint: endpoint,
          connectionTimeoutMillis: connectionTimeoutMillis,
          requestTimeoutMillis: requestTimeoutMillis,
          compression: compression,
          retryConfig: retryConfig,
        ),
      ),
    );
  }

  S2AccessTokens get accessTokens {
    return S2AccessTokens(_client);
  }

  S2Basins get basins {
    return S2Basins(_client);
  }

  S2Basin basin(String name) {
    return S2Basin(_client.basin(name: name));
  }
}
