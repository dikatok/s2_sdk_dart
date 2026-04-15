import 'dart:convert';

import 'package:s2_sdk_dart/src/rust/types.dart';

export 'access_tokens.dart';
export 'append_session.dart';
export "basin.dart";
export 'basins.dart';
export "client.dart";
export 'producer.dart';
export 'record.dart';
export "rust/error.dart";
export "stream.dart";
export 'streams.dart';

extension FormattedHeaderOnSequencedRecord on SequencedRecord {
  Map<String, String> get formattedHeaders => Map.fromEntries(
    headers.map((e) => MapEntry(utf8.decode(e.$1), utf8.decode(e.$2))),
  );
}
