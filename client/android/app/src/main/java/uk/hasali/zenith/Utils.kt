package uk.hasali.zenith

import android.content.Intent
import android.os.Build
import android.os.Bundle
import android.os.Parcelable

fun <T : Parcelable> Bundle.parcelable(key: String?, klass: Class<T>) = when {
    Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU -> getParcelable(key, klass)
    else -> @Suppress("DEPRECATION") getParcelable(key)
}

fun <T : Parcelable> Bundle.parcelableArrayList(key: String?, klass: Class<out T>) = when {
    Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU -> getParcelableArrayList(key, klass)
    else -> @Suppress("DEPRECATION") getParcelableArrayList(key)
}

fun <T : Parcelable> Intent.parcelableExtra(key: String?, klass: Class<out T>) = when {
    Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU -> getParcelableExtra(key, klass)
    else ->
        @Suppress("DEPRECATION") getParcelableExtra(key)
}
