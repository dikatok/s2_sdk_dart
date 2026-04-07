import 'package:s2_sdk_dart/src/rust/client.dart';

final class S2AccessTokens {
  final S2Client _client;

  S2AccessTokens(this._client);

  Future<String> issue(IssueAccessTokenInput input) {
    return _client.issueAccessToken(input: input);
  }

  Future<PageOfAccessTokenInfo> list(ListAccessTokensInput input) {
    return _client.listAccessTokens(input: input);
  }

  Future<Stream<AccessTokenInfo>> listAll(ListAllAccessTokensInput input) {
    return _client.listAllAccessTokens(input: input);
  }

  Future<void> revoke(String id) {
    return _client.revokeAccessToken(id: id);
  }
}
