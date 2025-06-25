import 'package:flutter/material.dart';
import '../services/api_service.dart';

class HomeScreen extends StatelessWidget {
  const HomeScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text("M2 - My Memory")),
      body: Center(
        child: ElevatedButton(
          onPressed: () async {
            final result = await ApiService.transcribe("audio.wav");
            print(result);
          },
          child: const Text("Start Transcription"),
        ),
      ),
    );
  }
}
