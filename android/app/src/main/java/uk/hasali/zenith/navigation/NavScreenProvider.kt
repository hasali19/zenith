package uk.hasali.zenith.navigation

import android.os.Bundle
import android.os.Parcelable
import androidx.compose.runtime.compositionLocalOf
import androidx.lifecycle.SavedStateHandle
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.android.components.ViewModelComponent
import kotlinx.parcelize.Parcelize
import kotlin.reflect.KProperty

private const val KEY_NAV_ENTRY_SCREEN_PROVIDER = "uk.hasali.zenith.navigation.NavEntry.screen"

@Parcelize
class NavScreenProvider(val parent: NavScreenProvider?, val screen: Parcelable) : Parcelable {
    inline operator fun <reified T> getValue(receiver: Any?, property: KProperty<*>): T {
        var current = this
        while (current.screen !is T) {
            val parent = current.parent
            if (parent == null) {
                throw IllegalArgumentException("No screen found for ${T::class.java}")
            } else {
                current = parent
            }
        }
        return current.screen as T
    }
}

val LocalNavScreenProvider = compositionLocalOf<NavScreenProvider?> { null }

@Module
@InstallIn(ViewModelComponent::class)
object NavScreenProviderModule {
    @Provides
    fun provideNavScreenProvider(savedStateHandle: SavedStateHandle): NavScreenProvider =
        savedStateHandle[KEY_NAV_ENTRY_SCREEN_PROVIDER]!!
}

fun Bundle.putNavScreenProvider(navScreenProvider: NavScreenProvider) {
    putParcelable(KEY_NAV_ENTRY_SCREEN_PROVIDER, navScreenProvider)
}
