package uk.co.hasali.zenith

import android.content.Intent
import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.setContent
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.launch
import uk.co.hasali.zenith.ui.ZenithTheme

class SetupActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContent {
            SetupScreen(settingsRepo = UserSettingsRepository.getInstance(this)) {
                startActivity(Intent(this, MainActivity::class.java))
                finish()
            }
        }
    }
}

@Composable
fun SetupScreen(settingsRepo: UserSettingsRepository, onFinish: () -> Unit) {
    val scope = rememberCoroutineScope()

    var address by remember { mutableStateOf("") }
    var useHttps by remember { mutableStateOf(false) }
    var loading by remember { mutableStateOf(false) }

    val onSubmit = {
        loading = true

        val url = when (useHttps) {
            true -> "https://$address"
            false -> "http://$address"
        }

        scope.launch {
            settingsRepo.setServerUrl(url)
            onFinish()
        }
    }

    ZenithTheme {
        Scaffold(
            topBar = {
                Column {
                    TopAppBar(title = { Text("Setup") })
                    if (loading) {
                        LinearProgressIndicator(modifier = Modifier.fillMaxWidth())
                    }
                }
            },
            bodyContent = {
                Box(modifier = Modifier.fillMaxSize()) {
                    Column(
                        modifier = Modifier.fillMaxWidth().align(Alignment.Center).padding(32.dp)
                    ) {
                        AddressTextField(
                            value = address,
                            onValueChange = { address = it },
                            modifier = Modifier.padding(vertical = 8.dp)
                        )

                        Row(modifier = Modifier.align(Alignment.End).padding(vertical = 8.dp)) {
                            Text("Use https")
                            Spacer(modifier = Modifier.width(8.dp))
                            Switch(checked = useHttps, onCheckedChange = { useHttps = it })
                        }

                        Button(
                            onClick = { onSubmit() },
                            modifier = Modifier.align(Alignment.End).padding(vertical = 8.dp)
                        ) {
                            Text("Done")
                        }
                    }
                }
            }
        )
    }
}

@Composable
fun AddressTextField(
    value: String,
    onValueChange: (String) -> Unit,
    modifier: Modifier = Modifier
) {
    TextField(
        label = { Text("Server address") },
        maxLines = 1,
        value = value,
        onValueChange = onValueChange,
        modifier = modifier.fillMaxWidth(),
        keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Uri)
    )
}
