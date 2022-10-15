import 'dart:convert';

import 'package:http/http.dart' as http;
import 'package:json_annotation/json_annotation.dart';

part 'github.g.dart';

@JsonSerializable(fieldRename: FieldRename.snake)
class GetActionsWorkflowRunsResponse {
  final List<ActionsWorkflowRun> workflowRuns;

  GetActionsWorkflowRunsResponse({required this.workflowRuns});

  factory GetActionsWorkflowRunsResponse.fromJson(Map<String, dynamic> json) =>
      _$GetActionsWorkflowRunsResponseFromJson(json);
}

@JsonSerializable(fieldRename: FieldRename.snake)
class ActionsWorkflowRun {
  final int id;
  final String status;
  final String? conclusion;
  final String headSha;
  final String headBranch;

  ActionsWorkflowRun({
    required this.id,
    required this.status,
    required this.conclusion,
    required this.headSha,
    required this.headBranch,
  });

  factory ActionsWorkflowRun.fromJson(Map<String, dynamic> json) =>
      _$ActionsWorkflowRunFromJson(json);
}

@JsonSerializable(fieldRename: FieldRename.snake)
class GetActionsWorkflowRunArtifactsResponse {
  List<ActionsWorkflowRunArtifact> artifacts;

  GetActionsWorkflowRunArtifactsResponse({required this.artifacts});

  factory GetActionsWorkflowRunArtifactsResponse.fromJson(
          Map<String, dynamic> json) =>
      _$GetActionsWorkflowRunArtifactsResponseFromJson(json);
}

@JsonSerializable(fieldRename: FieldRename.kebab)
class ActionsWorkflowRunArtifact {
  int id;
  String name;
  bool expired;

  ActionsWorkflowRunArtifact({
    required this.id,
    required this.name,
    required this.expired,
  });

  factory ActionsWorkflowRunArtifact.fromJson(Map<String, dynamic> json) =>
      _$ActionsWorkflowRunArtifactFromJson(json);
}

class GitHub {
  Future<GetActionsWorkflowRunsResponse> getActionsWorkflowRuns(
      int workflowId) async {
    return GetActionsWorkflowRunsResponse.fromJson(await _get(Uri.parse(
        'https://api.github.com/repos/hasali19/zenith/actions/workflows/$workflowId/runs')));
  }

  Future<GetActionsWorkflowRunArtifactsResponse> getActionsWorkflowRunArtifacts(
      int runId) async {
    return GetActionsWorkflowRunArtifactsResponse.fromJson(await _get(Uri.parse(
        "https://api.github.com/repos/hasali19/zenith/actions/runs/$runId/artifacts")));
  }

  Future<dynamic> _get(Uri uri) async {
    final res = await http.get(uri);
    if (res.statusCode >= 200 && res.statusCode < 300) {
      return jsonDecode(utf8.decode(res.bodyBytes));
    } else {
      throw Exception('Request returned status ${res.statusCode}');
    }
  }
}
