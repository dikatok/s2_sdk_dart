import 'dart:io';

import 'package:camera_desktop/camera_desktop.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:permission_handler/permission_handler.dart';
import 'package:video/features/app/bloc/app_cubit.dart';
import 'package:video/features/app/bloc/app_state.dart';
import 'package:video/features/enter_room/enter_room_screen.dart';
import 'package:video/features/room/bloc/room_cubit.dart';
import 'package:video/features/room/room_screen.dart';
import 'package:video/services/s2_service.dart';

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  if (Platform.isWindows || Platform.isLinux || Platform.isMacOS) {
    CameraDesktopPlugin.registerWith();
  }

  if (!Platform.isMacOS) {
    await [Permission.camera, Permission.microphone].request();
  }

  final s2 = S2Service();
  await s2.init("token", endpoint: "http://192.168.1.3:8080");

  runApp(MyApp(s2: s2));
}

class MyApp extends StatelessWidget {
  final S2Service s2;

  const MyApp({super.key, required this.s2});

  @override
  Widget build(BuildContext context) {
    return MultiBlocProvider(
      providers: [
        BlocProvider(create: (context) => AppCubit()),
        BlocProvider(create: (context) => RoomCubit(s2)),
      ],
      child: Builder(
        builder: (context) {
          return BlocBuilder(
            bloc: context.read<AppCubit>(),
            builder: (context, state) {
              return MaterialApp(
                title: "S2 Video",
                home: switch (state) {
                  NotJoinedRoom() => const EnterRoomScreen(),
                  JoinedRoom(:final user, :final room) => RoomScreen(
                    room: room,
                    user: user,
                  ),
                  _ => const Center(child: CircularProgressIndicator()),
                },
              );
            },
          );
        },
      ),
    );
  }
}
