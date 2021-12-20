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
    val status: String,
    val conclusion: String?,
    @SerialName("head_sha")
    val headSha: String,
    @SerialName("head_branch")
    val headBranch: String,
)

interface GitHubService {
    @GET("repos/hasali19/zenith/actions/workflows/{id}/runs")
    suspend fun getActionsWorkflowRuns(@Path("id") workflowId: Int): GetActionsWorkflowRunsResponse
}
