package uk.hasali.zenith

import android.app.Activity
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.stringPreferencesKey
import androidx.datastore.preferences.preferencesDataStore
import kotlinx.coroutines.flow.map

private val Activity.preferences by preferencesDataStore("settings")

class Preferences(context: Activity) {
    private val preferences = context.preferences

    object Keys {
        val SERVER_URL = stringPreferencesKey("server_url")
    }

    val serverUrl = preferences.data.map { it[Keys.SERVER_URL] }

    suspend fun setServerUrl(url: String) {
        preferences.edit {
            it[Keys.SERVER_URL] = url
        }
    }
}
