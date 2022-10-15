package uk.hasali.zenith

import android.content.Intent
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.gestures.detectTapGestures
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.input.pointer.pointerInput
import androidx.compose.ui.platform.LocalFocusManager
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import androidx.core.view.WindowCompat
import kotlinx.coroutines.launch
import uk.hasali.zenith.ui.AppTheme

class SelectServerActivity : ComponentActivity() {
    private lateinit var preferences: Preferences

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Enable drawing under the status bar
        WindowCompat.setDecorFitsSystemWindows(window, false)

        preferences = Preferences(this)

        setContent {
            val scope = rememberCoroutineScope()
            val focusManager = LocalFocusManager.current

            var address by remember { mutableStateOf("") }
            var https by remember { mutableStateOf(false) }

            AppTheme {
                Scaffold {
                    Column(
                        modifier = Modifier
                            .fillMaxSize()
                            .statusBarsPadding()
                            .navigationBarsPadding()
                            .imePadding()
                            .pointerInput(Unit) {
                                detectTapGestures {
                                    focusManager.clearFocus()
                                }
                            },
                    ) {
                        Box(
                            modifier = Modifier
                                .fillMaxWidth()
                                .weight(1f),
                        ) {
                            Text(
                                text = "Select server",
                                style = MaterialTheme.typography.h4,
                                modifier = Modifier.align(Alignment.Center),
                            )
                        }

                        Box(modifier = Modifier.fillMaxWidth()) {
                            Column(
                                horizontalAlignment = Alignment.CenterHorizontally,
                                verticalArrangement = Arrangement.Center,
                                modifier = Modifier
                                    .fillMaxWidth()
                                    .padding(horizontal = 32.dp),
                            ) {
                                TextField(
                                    value = address,
                                    label = { Text("Address") },
                                    singleLine = true,
                                    onValueChange = { address = it },
                                    modifier = Modifier.fillMaxWidth(),
                                    keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Uri)
                                )

                                Spacer(modifier = Modifier.height(8.dp))

                                Row(modifier = Modifier.align(Alignment.End)) {
                                    Text("Use https")
                                    Spacer(modifier = Modifier.width(8.dp))
                                    Switch(checked = https, onCheckedChange = { https = it })
                                }
                            }
                        }

                        Box(
                            modifier = Modifier
                                .fillMaxWidth()
                                .weight(1f),
                        ) {
                            Button(
                                modifier = Modifier.align(Alignment.Center),
                                onClick = {
                                    val protocol = when (https) {
                                        true -> "https"
                                        else -> "http"
                                    }

                                    scope.launch {
                                        preferences.setServerUrl("$protocol://$address")

                                        val intent = Intent(
                                            this@SelectServerActivity,
                                            MainActivity::class.java
                                        )

                                        startActivity(intent)
                                        finish()
                                    }
                                },
                            ) {
                                Text("Start")
                            }
                        }
                    }
                }
            }
        }
    }
}
