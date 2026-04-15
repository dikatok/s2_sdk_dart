import 'dart:async';

import 'package:bloc_presentation/bloc_presentation.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:video/features/room/bloc/room_event.dart';
import 'package:video/features/room/bloc/room_state.dart';
import 'package:video/misc/media_type.dart';
import 'package:video/services/s2_service.dart';

class RoomCubit extends Cubit<RoomState>
    with BlocPresentationMixin<RoomState, RoomEvent> {
  final S2Service _s2;

  StreamSubscription? _metadataSubscription;
  StreamSubscription? _participantsSubscription;
  final Map<String, StreamSubscription> _mediaSubscriptions = {};

  RoomCubit(this._s2) : super(RoomInitial());

  void joinRoom(String room, String user) async {
    emit(RoomJoining());
    try {
      final participants = await _s2.join(room, user);
      // print(participants);

      emit(RoomJoined(room: room, user: user, participants: participants));

      _metadataSubscription = _s2.listenMetadata(room).listen((data) {
        if (state is! RoomJoined) return;
        final type = data['type'];
        final otherUser = data['user'];
        if (user == otherUser) return;
        if (type == "join") {
          emitPresentation(UserJoined(otherUser));
        } else if (type == "leave") {
          emitPresentation(UserLeft(otherUser));
        }
      });

      _participantsSubscription = _s2.listenParticipants(room).listen((
        participants,
      ) {
        if (state is! RoomJoined) return;
        print("received participants");
        print(participants);

        for (final p in participants) {
          if (p == user) continue;
          if (_mediaSubscriptions.containsKey(p)) continue;
          _mediaSubscriptions[p] = _s2.listenUserMedia(room, p).listen((data) {
            if (state is! RoomJoined) return;
            emitPresentation(MediaReceived(p, data.$1, data.$2));
          });
        }

        final leftParticipants = (state as RoomJoined).participants
            .where((p) => !participants.contains(p))
            .toList();
        for (final p in leftParticipants) {
          _mediaSubscriptions[p]?.cancel();
          _mediaSubscriptions.remove(p);
        }

        emit(RoomJoined(room: room, user: user, participants: participants));
      });
    } catch (e, s) {
      debugPrintStack(stackTrace: s);
      emit(RoomFailedToJoin(e.toString()));
    }
  }

  void sendVideoFrame(Uint8List frame) {
    if (state is! RoomJoined) return;
    final s = state as RoomJoined;
    _s2.sendUserMedia(s.room, s.user, MediaType.video, frame);
  }

  void leaveRoom() {
    if (state is RoomJoined) {
      final s = state as RoomJoined;
      _s2.leaveRoom(s.room, s.user);
    }
    emit(RoomLeft());
    _cancelStreams();
  }

  void _cancelStreams() {
    _metadataSubscription?.cancel();
    _participantsSubscription?.cancel();
    for (final sub in _mediaSubscriptions.values) {
      sub.cancel();
    }
    _mediaSubscriptions.clear();
  }

  @override
  Future<void> close() {
    _cancelStreams();
    return super.close();
  }
}
