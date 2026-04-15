import 'package:equatable/equatable.dart';

abstract class RoomState extends Equatable {
  const RoomState();

  @override
  List<Object?> get props => [];
}

class RoomInitial extends RoomState {}

class RoomJoining extends RoomState {}

class RoomJoined extends RoomState {
  final String room;
  final String user;
  final List<String> participants;

  RoomJoined({
    required this.room,
    required this.user,
    required List<String> participants,
  }) : participants = [
         ...participants.where((p) => p == user),
         ...participants.where((p) => p != user),
       ];

  @override
  List<Object?> get props => [room, user, participants];
}

class RoomFailedToJoin extends RoomState {
  final String message;

  const RoomFailedToJoin(this.message);

  @override
  List<Object?> get props => [message];
}

class RoomLeft extends RoomState {}
