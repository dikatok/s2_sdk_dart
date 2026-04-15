import "package:equatable/equatable.dart";

final class AppState extends Equatable {
  @override
  List<Object> get props => [];
}

final class NotJoinedRoom extends AppState {}

final class JoinedRoom extends AppState {
  JoinedRoom({required this.room, required this.user}) : super();

  final String room;
  final String user;

  @override
  List<Object> get props => [room, user];
}
