// imageLib -> Image package from https://pub.dartlang.org/packages/image
import 'package:camera/camera.dart';
import 'package:flutter/foundation.dart';
import 'package:image/image.dart' as image;

Uint8List? convertCameraImagetoJpg(CameraImage cameraImage) {
  try {
    if (cameraImage.format.group == ImageFormatGroup.yuv420) {
      // android
      final img = image.copyRotate(_convertYUV420(cameraImage), angle: -90);
      return Uint8List.fromList(image.encodeJpg(img, quality: 10));
    } else if (cameraImage.format.group == ImageFormatGroup.bgra8888) {
      final img = image.Image.fromBytes(
        width: cameraImage.width,
        height: cameraImage.height,
        bytes: cameraImage.planes[0].bytes.buffer,
        order: image.ChannelOrder.bgra,
      );
      return Uint8List.fromList(image.encodeJpg(img, quality: 10));
    }
  } catch (e, s) {
    debugPrintStack(stackTrace: s);
  }
  return null;
}

image.Image _convertYUV420(CameraImage cameraImage) {
  final width = cameraImage.width;
  final height = cameraImage.height;
  final uvRowStride = cameraImage.planes[1].bytesPerRow;
  final uvPixelStride = cameraImage.planes[1].bytesPerPixel!;

  final out = image.Image(width: width, height: height);

  for (var y = 0; y < height; y++) {
    for (var x = 0; x < width; x++) {
      final uvIndex =
          uvPixelStride * (x / 2).floor() + uvRowStride * (y / 2).floor();
      final index = y * width + x;

      final yp = cameraImage.planes[0].bytes[index];
      final up = cameraImage.planes[1].bytes[uvIndex];
      final vp = cameraImage.planes[2].bytes[uvIndex];

      final r = (yp + vp * 1436 / 1024 - 179).round().clamp(0, 255);
      final g = (yp - up * 46549 / 131072 + 44 - vp * 93604 / 131072 + 91)
          .round()
          .clamp(0, 255);
      final b = (yp + up * 1814 / 1024 - 227).round().clamp(0, 255);

      out.setPixelRgb(x, y, r, g, b);
    }
  }
  return out;
}
