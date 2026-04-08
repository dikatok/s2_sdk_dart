# s2_sdk_dart

Dart SDK for S2, a serverless data store for streams, built on top of the Rust official SDK.

S2 is a managed service that provides unlimited, durable streams.

Streams can be appended to, with all new records added to the tail of the stream. You can read from any portion of a stream – indexing by record sequence number, or timestamp – and follow updates live.

## Getting Started

* Make sure [Flutter](https://docs.flutter.dev/install) and [rust](https://rust-lang.org/tools/install/) are installed.
* Add `s2_sdk_dart` to your flutter project's `pubspec.yaml`, or checkout the example folder. 

## Let's Start Streaming

```dart
final s2 = await S2.create("token");

await s2.basins.create("test-basin");
await s2.basin("test-basin").streams.create("counter");

final stream = s2.basin("test-basin").stream("counter");

final readSession = await stream.readSession();

final Stream<int> counterStream = readSession.map((record) {
	return jsonDecode(utf8.decode(record.body))["delta"];
});
counterStream.listen((event) {
	setState(() {
		_counter += event;
	});
});

// Somewhere else
await stream.append([
	S2AppendRecord.fromObject({"delta": 1}),
]);
```