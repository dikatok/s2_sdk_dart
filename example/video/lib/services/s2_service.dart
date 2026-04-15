import 'dart:convert';

import 'package:flutter/foundation.dart';
import 'package:s2_sdk_dart/s2_sdk_dart.dart';
import 'package:video/misc/media_type.dart';

const _basinName = 'video-calls';

class S2Service {
  late S2 _client;

  Future<void> init(String token, {String? endpoint}) async {
    _client = await S2.create(token, endpoint: endpoint);
  }

  S2Stream _getMetadataStream(String room) {
    return _client.basin(_basinName).stream('room/$room/metadata');
  }

  S2Stream _getUserStream(String room, String user) {
    return _client.basin(_basinName).stream('room/$room/user/$user');
  }

  S2Stream _getParticipantsStream(String room) {
    return _client.basin(_basinName).stream('room/$room/participants');
  }

  // Returns list of participants
  Future<List<String>> join(String room, String user) async {
    await _getMetadataStream(room).append([
      S2AppendRecord.fromObject({'type': 'join', 'user': user}),
    ]);
    final participantStream = _getParticipantsStream(room);
    final batch = await participantStream.read(
      start: ReadStart(from: ReadFrom.tailOffset(1)),
      stop: ReadStop(waitSecs: 1),
    );
    final data = batch.records.isEmpty
        ? {'participants': []}
        : jsonDecode(utf8.decode(batch.records.last.body));
    final participants = List<String>.from(data['participants']);
    if (!participants.contains(user)) {
      participants.add(user);
      await participantStream.append([
        S2AppendRecord.fromObject({'participants': participants}),
      ]);
    }
    return participants;
  }

  Future<void> leaveRoom(String room, String user) async {
    await _getMetadataStream(room).append([
      S2AppendRecord.fromObject({'type': 'leave', 'user': user}),
    ]);
    final participantStream = _getParticipantsStream(room);
    final batch = await participantStream.read(
      start: ReadStart(from: ReadFrom.tailOffset(1)),
      stop: ReadStop(waitSecs: 1),
    );
    if (batch.records.isNotEmpty) {
      {
        final data = jsonDecode(utf8.decode(batch.records.last.body));
        final participants = List<String>.from(data['participants']);
        if (participants.contains(user)) {
          participants.remove(user);
          await participantStream.append([
            S2AppendRecord.fromObject({'participants': participants}),
          ]);
        }
      }
    }
  }

  Stream<Map<String, dynamic>> listenMetadata(String room) async* {
    final stream = _getMetadataStream(room);
    final session = await stream.readSession(
      start: ReadStart(from: ReadFrom.tailOffset(1)),
    );
    yield* session.map((record) => jsonDecode(utf8.decode(record.body)));
  }

  Stream<List<String>> listenParticipants(String room) async* {
    final stream = _getParticipantsStream(room);
    final session = await stream.readSession(
      start: ReadStart(from: ReadFrom.tailOffset(1)),
    );
    yield* session.map((record) {
      final data = jsonDecode(utf8.decode(record.body));
      return List<String>.from(data['participants'] ?? []);
    });
  }

  Stream<(MediaType, Uint8List)> listenUserMedia(
    String room,
    String user,
  ) async* {
    final stream = _getUserStream(room, user);
    final session = await stream.readSession(
      start: ReadStart(from: ReadFrom.tailOffset(1)),
    );
    yield* session.map(
      (record) => (
        MediaType.values.byName(record.formattedHeaders['content-type']!),
        record.body,
      ),
    );
  }

  Future<void> sendUserMedia(
    String room,
    String user,
    MediaType type,
    Uint8List media,
  ) async {
    final s = _getUserStream(room, user);
    s.append([
      S2AppendRecord(
        body: media,
        headers: {"content-type": type.name},
        timestamp: DateTime.now().millisecondsSinceEpoch,
      ),
    ]);
  }
}
