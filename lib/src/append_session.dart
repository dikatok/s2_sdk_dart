import 'package:s2_sdk_dart/src/rust/types.dart';

import 'rust/append_session.dart' as inner;

final class S2AppendSession {
  final inner.S2AppendSession _appendSession;

  S2AppendSession(this._appendSession);

  Future<inner.BatchSubmitTicket> append(AppendInput input) {
    return _appendSession.submit(record: input);
  }

  Future<void> close() {
    return _appendSession.close();
  }
}
