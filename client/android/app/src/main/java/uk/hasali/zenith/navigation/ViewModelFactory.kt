package uk.hasali.zenith.navigation

import android.app.Activity
import android.os.Bundle
import androidx.compose.runtime.Composable
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.platform.LocalSavedStateRegistryOwner
import androidx.lifecycle.*
import androidx.lifecycle.viewmodel.compose.viewModel
import dagger.hilt.EntryPoint
import dagger.hilt.EntryPoints
import dagger.hilt.InstallIn
import dagger.hilt.android.components.ActivityComponent
import dagger.hilt.android.components.ViewModelComponent
import dagger.hilt.android.internal.builders.ViewModelComponentBuilder
import dagger.hilt.android.internal.lifecycle.HiltViewModelMap
import javax.inject.Provider

@EntryPoint
@InstallIn(ActivityComponent::class)
interface ActivityCreatorEntryPoint {
    val viewModelComponentBuilder: ViewModelComponentBuilder
}

@EntryPoint
@InstallIn(ViewModelComponent::class)
interface ViewModelFactoriesEntryPoint {
    @get:HiltViewModelMap
    val hiltViewModelMap: Map<String, Provider<ViewModel>>
}

@Composable
fun hiltViewModelFactory(): ViewModelProvider.Factory {
    val activity = LocalContext.current as Activity
    val navScreenProvider = LocalNavScreenProvider.current
    val savedStateRegistryOwner = LocalSavedStateRegistryOwner.current
    val defaultArgs = Bundle().apply {
        if (navScreenProvider != null) {
            putNavScreenProvider(navScreenProvider)
        }
    }

    return object : AbstractSavedStateViewModelFactory(savedStateRegistryOwner, defaultArgs) {
        override fun <T : ViewModel?> create(
            key: String,
            modelClass: Class<T>,
            handle: SavedStateHandle,
        ): T {
            val viewModelComponent =
                EntryPoints.get(activity, ActivityCreatorEntryPoint::class.java)
                    .viewModelComponentBuilder
                    .savedStateHandle(handle)
                    .build()

            val model =
                EntryPoints.get(viewModelComponent, ViewModelFactoriesEntryPoint::class.java)
                    .hiltViewModelMap[modelClass.name]!!
                    .get()

            @Suppress("UNCHECKED_CAST")
            return model as T
        }
    }
}

@Composable
inline fun <reified T : ViewModel> hiltViewModel(): T =
    viewModel(factory = hiltViewModelFactory())
