// GENERATED CODE - DO NOT MODIFY BY HAND

part of 'github.dart';

// **************************************************************************
// JsonSerializableGenerator
// **************************************************************************

GetActionsWorkflowRunsResponse _$GetActionsWorkflowRunsResponseFromJson(
        Map<String, dynamic> json) =>
    GetActionsWorkflowRunsResponse(
      workflowRuns: (json['workflow_runs'] as List<dynamic>)
          .map((e) => ActionsWorkflowRun.fromJson(e as Map<String, dynamic>))
          .toList(),
    );

Map<String, dynamic> _$GetActionsWorkflowRunsResponseToJson(
        GetActionsWorkflowRunsResponse instance) =>
    <String, dynamic>{
      'workflow_runs': instance.workflowRuns,
    };

ActionsWorkflowRun _$ActionsWorkflowRunFromJson(Map<String, dynamic> json) =>
    ActionsWorkflowRun(
      id: json['id'] as int,
      status: json['status'] as String,
      conclusion: json['conclusion'] as String?,
      headSha: json['head_sha'] as String,
      headBranch: json['head_branch'] as String,
    );

Map<String, dynamic> _$ActionsWorkflowRunToJson(ActionsWorkflowRun instance) =>
    <String, dynamic>{
      'id': instance.id,
      'status': instance.status,
      'conclusion': instance.conclusion,
      'head_sha': instance.headSha,
      'head_branch': instance.headBranch,
    };

GetActionsWorkflowRunArtifactsResponse
    _$GetActionsWorkflowRunArtifactsResponseFromJson(
            Map<String, dynamic> json) =>
        GetActionsWorkflowRunArtifactsResponse(
          artifacts: (json['artifacts'] as List<dynamic>)
              .map((e) => ActionsWorkflowRunArtifact.fromJson(
                  e as Map<String, dynamic>))
              .toList(),
        );

Map<String, dynamic> _$GetActionsWorkflowRunArtifactsResponseToJson(
        GetActionsWorkflowRunArtifactsResponse instance) =>
    <String, dynamic>{
      'artifacts': instance.artifacts,
    };

ActionsWorkflowRunArtifact _$ActionsWorkflowRunArtifactFromJson(
        Map<String, dynamic> json) =>
    ActionsWorkflowRunArtifact(
      id: json['id'] as int,
      name: json['name'] as String,
      expired: json['expired'] as bool,
    );

Map<String, dynamic> _$ActionsWorkflowRunArtifactToJson(
        ActionsWorkflowRunArtifact instance) =>
    <String, dynamic>{
      'id': instance.id,
      'name': instance.name,
      'expired': instance.expired,
    };
