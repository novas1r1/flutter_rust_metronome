import 'package:flutter/material.dart';

import 'ffi.dart' if (dart.library.html) 'ffi_web.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: const MyHomePage(title: 'Flutter Demo Home Page'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({Key? key, required this.title}) : super(key: key);

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  @override
  void initState() {
    super.initState();
    api.initLogger();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('HELLO'),
      ),
      body: Column(
        children: [
          Row(
            children: [
              IconButton(
                onPressed: () async {
                  await api.setBpm(bpm: 60);
                  await api.play();
                },
                icon: const Icon(
                  Icons.play_circle_fill,
                  size: 42,
                  color: Colors.blue,
                ),
              ),
              IconButton(
                onPressed: () async {
                  await api.stop();
                },
                icon: const Icon(
                  Icons.stop_circle,
                  size: 42,
                  color: Colors.blue,
                ),
              ),
            ],
          ),
        ],
      ),
    );
  }
}
