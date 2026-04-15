import 'dart:convert';
import 'dart:typed_data';

class S2AppendRecord {
  final Uint8List body;
  final Map<String, String>? headers;
  final int? timestamp;

  S2AppendRecord({
    required this.body,
    required this.headers,
    required this.timestamp,
  });

  factory S2AppendRecord.fromString(
    String body, {
    Map<String, String>? headers,
    int? timestamp,
  }) {
    return S2AppendRecord(
      body: utf8.encode(body),
      headers: headers,
      timestamp: timestamp,
    );
  }

  factory S2AppendRecord.fromObject(
    Map<String, dynamic> body, {
    Map<String, String>? headers,
    int? timestamp,
  }) {
    return S2AppendRecord(
      body: utf8.encode(jsonEncode(body)),
      headers: headers,
      timestamp: timestamp,
    );
  }

  List<(Uint8List, Uint8List)> getHeaderInBytes() {
    return headers?.entries
            .map((e) => (utf8.encode(e.key), utf8.encode(e.value)))
            .toList() ??
        [];
  }
}
