import 'dart:convert';
import 'dart:typed_data';

class S2AppendRecord {
  final Uint8List body;
  final List<(Uint8List, Uint8List)>? headers;
  final int? timestamp;

  S2AppendRecord._({
    required this.body,
    required this.headers,
    required this.timestamp,
  });

  factory S2AppendRecord.fromObject(
    Map<String, dynamic> body, {
    Map<String, dynamic>? headers,
    int? timestamp,
  }) {
    return S2AppendRecord._(
      body: utf8.encode(jsonEncode(body)),
      headers: headers?.entries
          .map((e) => (utf8.encode(e.key), utf8.encode(e.value)))
          .toList(),
      timestamp: timestamp,
    );
  }
}
