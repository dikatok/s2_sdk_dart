import 'package:s2_sdk_dart/src/rust/client.dart';

export 'rust/client.dart'
    show AccessTokenScopeInput, OperationGroupPermissions, ReadWritePermissions;
export 'rust/types.dart' show ResourceSet, Operation;

final class S2AccessTokens {
  final S2Client _client;

  S2AccessTokens(this._client);

  Future<String> issue(
    String id, {
    int? expiresAt,
    bool autoPrefixStreams = false,
    required AccessTokenScopeInput scope,
  }) {
    return _client.issueAccessToken(
      input: IssueAccessTokenInput(
        id: id,
        expiresAt: expiresAt,
        autoPrefixStreams: autoPrefixStreams,
        scope: scope,
      ),
    );
  }

  Future<PageOfAccessTokenInfo> list({
    String? prefix,
    String? startAfter,
    int? limit,
  }) {
    return _client.listAccessTokens(
      input: ListAccessTokensInput(
        prefix: prefix,
        startAfter: startAfter,
        limit: limit,
      ),
    );
  }

  Future<Stream<AccessTokenInfo>> listAll({
    String? prefix,
    String? startAfter,
  }) {
    return _client.listAllAccessTokens(
      input: ListAllAccessTokensInput(prefix: prefix, startAfter: startAfter),
    );
  }

  Future<void> revoke(String id) {
    return _client.revokeAccessToken(id: id);
  }
}
