package sapling.foliage.gql

import android.content.Context
import androidx.lifecycle.ViewModel
import androidx.lifecycle.ViewModelProvider
import com.apollographql.apollo.ApolloClient

class ViewModelFactory(private val context: Context) : ViewModelProvider.Factory {
    override fun <T : ViewModel> create(modelClass: Class<T>): T {
        if (modelClass.isAssignableFrom(GraphQLViewModel::class.java)) {
            val apolloClient =
                ApolloClient.Builder().serverUrl("https://sapling.geigr.dev/gql").build()

            @Suppress("UNCHECKED_CAST")
            return GraphQLViewModel(apolloClient) as T
        }

        throw IllegalArgumentException("Unknown ViewModel class")
    }
}