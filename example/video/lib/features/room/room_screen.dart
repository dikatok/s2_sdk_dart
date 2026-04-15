import 'dart:async';

import 'package:bloc_presentation/bloc_presentation.dart';
import 'package:camera/camera.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:video/features/app/bloc/app_cubit.dart';
import 'package:video/features/room/bloc/room_cubit.dart';
import 'package:video/features/room/bloc/room_event.dart';
import 'package:video/features/room/bloc/room_state.dart';
import 'package:video/misc/image.dart';
import 'package:video/misc/media_type.dart';

class RoomScreen extends StatefulWidget {
  const RoomScreen({super.key, required this.room, required this.user});

  final String room;
  final String user;

  @override
  State<RoomScreen> createState() => _RoomScreenState();
}

class _RoomScreenState extends State<RoomScreen> {
  CameraController? _controller;
  final Map<String, Uint8List> _remoteFrames = {};

  @override
  void initState() {
    super.initState();
    _initCamera();
    context.read<RoomCubit>().joinRoom(widget.room, widget.user);
  }

  Future<void> _initCamera() async {
    final cameras = await availableCameras();
    if (cameras.isEmpty) return;

    final camera = cameras.firstWhere(
      (c) => c.lensDirection == CameraLensDirection.front,
      orElse: () => cameras.first,
    );

    _controller = CameraController(
      camera,
      ResolutionPreset.low,
      enableAudio: false,
      fps: 5,
      imageFormatGroup: ImageFormatGroup.bgra8888,
    );

    try {
      await _controller!.initialize();
      await _controller!.startImageStream(_handleImageStream);
    } catch (e) {
      debugPrint(e.toString());
    }

    setState(() {});
  }

  void _handleImageStream(CameraImage image) async {
    try {
      final frame = await compute(convertCameraImagetoJpg, image);
      if (frame == null) return;
      if (!mounted) return;
      context.read<RoomCubit>().sendVideoFrame(frame);
    } catch (e, s) {
      debugPrint(e.toString());
      debugPrintStack(stackTrace: s);
    }
  }

  @override
  void dispose() {
    _controller?.stopImageStream();
    _controller?.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: Text('Room: ${widget.room}')),
      floatingActionButtonLocation: FloatingActionButtonLocation.centerDocked,
      floatingActionButton: FloatingActionButton(
        backgroundColor: Colors.red,
        shape: const CircleBorder(),
        onPressed: () async {
          context.read<RoomCubit>().leaveRoom();
        },
        tooltip: 'End Call',
        child: const Icon(Icons.call_end, color: Colors.white),
      ),
      body: Builder(
        builder: (context) {
          return BlocPresentationListener(
            bloc: context.read<RoomCubit>(),
            listener: (context, event) {
              switch (event) {
                case UserJoined(:final user):
                  ScaffoldMessenger.of(
                    context,
                  ).showSnackBar(SnackBar(content: Text('$user joined')));
                case UserLeft(:final user):
                  ScaffoldMessenger.of(
                    context,
                  ).showSnackBar(SnackBar(content: Text('$user left')));
                case MediaReceived(:final user, :final type, :final media):
                  if (type == MediaType.video) {
                    setState(() {
                      _remoteFrames[user] = media;
                    });
                  }
              }
            },
            child: BlocConsumer<RoomCubit, RoomState>(
              listener: (context, state) {
                if (state is RoomLeft) {
                  context.read<AppCubit>().leaveRoom();
                }
              },
              builder: (context, state) {
                if (state is RoomFailedToJoin) {
                  return Center(child: Text(state.message));
                }

                if (state is! RoomJoined) {
                  return Center(child: CircularProgressIndicator());
                }

                final participants = state.participants;

                return Column(
                  children: [
                    Container(
                      padding: const EdgeInsets.all(16),
                      color: Colors.grey[200],
                      child: Row(
                        children: [
                          const Icon(Icons.people),
                          const SizedBox(width: 8),
                          Text('${participants.length} Participant(s)'),
                          const Spacer(),
                          Text('Logged in as: ${state.user}'),
                        ],
                      ),
                    ),
                    Expanded(
                      child: GridView.builder(
                        padding: const EdgeInsets.all(12),
                        gridDelegate:
                            const SliverGridDelegateWithFixedCrossAxisCount(
                              crossAxisCount: 2,
                              crossAxisSpacing: 12,
                              mainAxisSpacing: 12,
                              childAspectRatio: 1,
                            ),
                        itemCount: participants.length,
                        itemBuilder: (context, index) {
                          final user = participants[index];
                          final isMe = user == state.user;
                          return Container(
                            decoration: BoxDecoration(
                              color: Colors.black87,
                              border: Border.all(
                                color: Colors.white24,
                                width: 1,
                              ),
                            ),
                            clipBehavior: Clip.antiAlias,
                            child: Stack(
                              fit: StackFit.expand,
                              children: [
                                if (isMe && _controller != null)
                                  CameraPreview(_controller!)
                                else if (!isMe &&
                                    _remoteFrames.containsKey(user))
                                  Image.memory(
                                    _remoteFrames[user]!,
                                    fit: BoxFit.cover,
                                    gaplessPlayback: true,
                                  )
                                else
                                  const Center(
                                    child: Icon(
                                      Icons.videocam_off,
                                      color: Colors.white54,
                                      size: 48,
                                    ),
                                  ),
                                Positioned(
                                  bottom: 8,
                                  left: 8,
                                  child: Container(
                                    padding: const EdgeInsets.symmetric(
                                      horizontal: 8,
                                      vertical: 4,
                                    ),
                                    decoration: BoxDecoration(
                                      color: Colors.black54,
                                    ),
                                    child: Text(
                                      isMe ? 'You' : user,
                                      style: const TextStyle(
                                        color: Colors.white,
                                        fontSize: 12,
                                      ),
                                    ),
                                  ),
                                ),
                              ],
                            ),
                          );
                        },
                      ),
                    ),
                  ],
                );
              },
            ),
          );
        },
      ),
    );
  }
}
