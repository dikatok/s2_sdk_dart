enum MediaType { audio, video }

extension ContentType on MediaType {
  String get contentType {
    switch (this) {
      case MediaType.audio:
        return 'audio';
      case MediaType.video:
        return 'video';
    }
  }
}
