import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:s2_sdk_dart/s2_sdk_dart.dart';

late S2 s2;

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  s2 = await S2.create(
    "token",
    endpoint: "http://10.0.2.2:8080",
  ); // for android emulator, otherwise use localhost
  try {
    await s2.basins.create("test-basin");
    await s2.basin("test-basin").streams.create("counter");
  } on S2Error catch (e) {
    debugPrint(e.toString());
  }
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(colorScheme: .fromSeed(seedColor: Colors.deepPurple)),
      home: const MyHomePage(title: 'Flutter Demo Home Page'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  S2Stream stream = s2.basin("test-basin").stream("counter");
  Stream<int>? counterStream;

  int _counter = 0;

  @override
  void initState() {
    stream.readSession().then((s) {
      counterStream = s.map((record) {
        return jsonDecode(utf8.decode(record.body))["delta"];
      });
      counterStream!.listen((event) {
        setState(() {
          _counter += event;
        });
      });
    });
    super.initState();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text(widget.title),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: .center,
          children: [
            const Text('You have pushed the button this many times:'),
            Text(
              '$_counter',
              style: Theme.of(context).textTheme.headlineMedium,
            ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () async {
          await stream.append([
            S2AppendRecord.fromObject({"delta": 1}),
          ]);
        },
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ),
    );
  }
}
