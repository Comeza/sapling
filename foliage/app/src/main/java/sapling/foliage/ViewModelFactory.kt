package sapling.foliage

import android.content.Context
import android.util.Log
import androidx.lifecycle.ViewModel
import androidx.lifecycle.ViewModelProvider

class ViewModelFactory(private val context: Context) : ViewModelProvider.Factory {
    override fun <T : ViewModel> create(modelClass: Class<T>): T {
        if (modelClass.isAssignableFrom(GraphQLViewModel::class.java)) {
            val apolloClient = GraphQLClient("http://10.2.3.67:3000/gql").create(context)

            @Suppress("UNCHECKED_CAST")
            return GraphQLViewModel(apolloClient) as T
        }

        throw IllegalArgumentException("Unknown ViewModel class")
    }
}