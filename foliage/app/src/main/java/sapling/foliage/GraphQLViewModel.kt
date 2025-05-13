package sapling.foliage

import android.util.Log
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.apollographql.apollo.ApolloClient
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch

class GraphQLViewModel(private val apolloClient: ApolloClient) : ViewModel() {
    private val _uiState = MutableStateFlow("Loading...")
    val uiState: StateFlow<String> = _uiState;

    fun fetchData() {
        viewModelScope.launch {
            try {
                val response = apolloClient.query(ProductsQuery()).execute()
                _uiState.value = response.dataOrThrow().products.toString()
            } catch (e: Exception) {
                Log.e("GraphQLViewModel", "Error fetching data", e)
                _uiState.value = "Error: ${e.message}"
            }
        }
    }
}