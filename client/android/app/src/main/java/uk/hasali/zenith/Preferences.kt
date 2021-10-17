package uk.hasali.zenith

import android.content.Context
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.stringPreferencesKey
import androidx.datastore.preferences.preferencesDataStore
import dagger.hilt.android.qualifiers.ApplicationContext
import kotlinx.coroutines.flow.map
import javax.inject.Inject

private val Context.preferences by preferencesDataStore("settings")

class Preferences @Inject constructor(@ApplicationContext context: Context) {
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
