package uk.hasali.zenith

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import retrofit2.http.GET
import retrofit2.http.Path

@Serializable
data class GetActionsWorkflowRunsResponse(
    @SerialName("workflow_runs")
    val workflowRuns: List<ActionsWorkflowRun>,
)

@Serializable
data class ActionsWorkflowRun(
    val id: Long,
    val status: String,
    val conclusion: String?,
    @SerialName("head_sha")
    val headSha: String,
    @SerialName("head_branch")
    val headBranch: String,
)

@Serializable
data class GetActionsWorkflowRunArtifactsResponse(
    val artifacts: List<ActionsWorkflowRunArtifact>,
)

@Serializable
data class ActionsWorkflowRunArtifact(
    val id: Long,
    val name: String,
    val expired: Boolean,
)

interface GitHubService {
    @GET("repos/hasali19/zenith/actions/workflows/{id}/runs")
    suspend fun getActionsWorkflowRuns(@Path("id") workflowId: Long): GetActionsWorkflowRunsResponse

    @GET("repos/hasali19/zenith/actions/runs/{id}/artifacts")
    suspend fun getActionsWorkflowRunArtifacts(@Path("id") runId: Long): GetActionsWorkflowRunArtifactsResponse
}
