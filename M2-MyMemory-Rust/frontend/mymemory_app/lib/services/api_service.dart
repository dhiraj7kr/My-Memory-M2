import 'dart:convert';
import 'package:http/http.dart' as http;

class ApiService {
  static const String _baseUrl = "http://localhost:8000";

  static Future<String> transcribe(String filePath) async {
    final response = await http.post(
      Uri.parse("$_baseUrl/transcribe"),
      headers: {'Content-Type': 'application/json'},
      body: jsonEncode({"file_path": filePath}),
    );
    return jsonDecode(response.body)['transcript'];
  }
}
