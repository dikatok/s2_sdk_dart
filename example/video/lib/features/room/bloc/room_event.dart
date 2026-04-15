import 'dart:typed_data';

import 'package:video/misc/media_type.dart';

sealed class RoomEvent {}

class UserJoined extends RoomEvent {
  final String user;
  UserJoined(this.user);
}

class MediaReceived extends RoomEvent {
  final String user;
  final MediaType type;
  final Uint8List media;
  MediaReceived(this.user, this.type, this.media);
}

class UserLeft extends RoomEvent {
  final String user;
  UserLeft(this.user);
}
