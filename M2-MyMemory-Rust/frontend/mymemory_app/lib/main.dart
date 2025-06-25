import 'package:flutter/material.dart';
import 'screens/home.dart';

void main() {
  runApp(const MyMemoryApp());
}

class MyMemoryApp extends StatelessWidget {
  const MyMemoryApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'My Memory',
      theme: ThemeData(primarySwatch: Colors.blue),
      home: const HomeScreen(),
    );
  }
}
