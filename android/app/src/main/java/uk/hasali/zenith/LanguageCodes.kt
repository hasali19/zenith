package uk.hasali.zenith

import android.content.Context
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.decodeFromStream
import java.util.*

object LanguageCodes {
    @Serializable
    private data class Entry(
        val alpha2: String,
        @SerialName("alpha3-b")
        val alpha3_b: String,
    )

    private val json = Json { ignoreUnknownKeys = true }
    private val entries = mutableMapOf<String, String>()

    @OptIn(ExperimentalSerializationApi::class)
    fun init(context: Context) {
        val entries = context.assets.open("language-codes.json").use {
            json.decodeFromStream<List<Entry>>(it)
        }
        this.entries.putAll(entries.map { Pair(it.alpha3_b, it.alpha2) })
    }

    fun getAlpha3(alpha2: String): String? {
        return entries[alpha2]
    }

    fun getDisplayNameForCode(code: String?, default: String = ""): String {
        if (code == null) return default
        // Locale requires alpha2 code for languages that have both
        // so we need to convert it first if possible
        val tag = getAlpha3(code) ?: code
        val locale = Locale.forLanguageTag(tag)
        return locale.displayName
    }
}
