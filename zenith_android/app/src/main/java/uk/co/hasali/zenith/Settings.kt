package uk.co.hasali.zenith

import android.content.Context
import androidx.datastore.core.DataStore
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.stringPreferencesKey
import androidx.datastore.preferences.createDataStore
import kotlinx.coroutines.flow.map

data class UserSettings(
    val serverUrl: String?
)

class UserSettingsRepository private constructor(context: Context) {

    private val store: DataStore<Preferences> = context.createDataStore(name = "settings")

    private object PreferenceKeys {
        val ZENITH_SERVER_URL = stringPreferencesKey("server_url")
    }

    val settings = store.data.map { preferences ->
        UserSettings(
            serverUrl = preferences[PreferenceKeys.ZENITH_SERVER_URL]
        )
    }

    suspend fun setServerUrl(serverUrl: String) {
        store.edit {
            it[PreferenceKeys.ZENITH_SERVER_URL] = serverUrl
        }
    }

    companion object {
        @Volatile
        private var INSTANCE: UserSettingsRepository? = null

        fun getInstance(context: Context): UserSettingsRepository {
            return INSTANCE ?: synchronized(this) {
                UserSettingsRepository(context).also { instance ->
                    INSTANCE = instance
                }
            }
        }
    }
}
