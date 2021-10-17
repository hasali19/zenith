package uk.hasali.zenith.navigation

import androidx.lifecycle.ViewModel
import androidx.lifecycle.ViewModelStore

class ViewModelStoreProvider : ViewModel() {
    private val viewModelStores = hashMapOf<Any, ViewModelStore>()

    fun get(key: Any): ViewModelStore {
        return viewModelStores.getOrPut(key) {
            ViewModelStore()
        }
    }

    fun clear(key: Any) {
        viewModelStores.remove(key)
            ?.clear()
    }
}
