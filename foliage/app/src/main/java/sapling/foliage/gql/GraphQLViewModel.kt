package sapling.foliage.gql

import android.util.Log
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.apollographql.apollo.ApolloClient
import com.apollographql.apollo.api.Query
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch
import sapling.foliage.ProductsQuery


class GraphQLViewModel<T : Query<D>, D : Query.Data>(private val apolloClient: ApolloClient) :
    ViewModel() {
    private val _uiState = MutableStateFlow<D?>(null)
    val uiState: StateFlow<D?> = _uiState;

    fun fetch(query: T) {
        viewModelScope.launch {
            ProductsQuery
            try {
                val response = apolloClient.query(query).execute()
                val data = response.dataOrThrow()
                _uiState.value = data;
            } catch (e: Exception) {
                Log.e("GraphQLViewModel", "Error fetching data", e)
            }
        }
    }
}