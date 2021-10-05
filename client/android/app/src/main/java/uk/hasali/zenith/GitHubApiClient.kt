package uk.hasali.zenith

import io.ktor.client.*
import io.ktor.client.request.*
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

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

class GitHubApiClient(private val client: HttpClient) {
    suspend fun getActionsWorkflowRuns(workflowId: Int): GetActionsWorkflowRunsResponse {
        return client.get("https://api.github.com/repos/hasali19/zenith/actions/workflows/$workflowId/runs")
    }
}
