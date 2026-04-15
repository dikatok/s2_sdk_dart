import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:video/features/app/bloc/app_state.dart';

class AppCubit extends Cubit<AppState> {
  AppCubit() : super(NotJoinedRoom());

  void joinRoom({required String room, required String user}) {
    emit(JoinedRoom(room: room, user: user));
  }

  void leaveRoom() {
    emit(NotJoinedRoom());
  }
}
