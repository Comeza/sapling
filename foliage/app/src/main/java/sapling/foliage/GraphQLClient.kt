package sapling.foliage

import android.content.Context
import com.apollographql.apollo.ApolloClient
import com.apollographql.apollo.network.okHttpClient
import okhttp3.OkHttpClient

class GraphQLClient(private val url: String) {

    fun create(context: Context): ApolloClient {
        val okHttpClient = OkHttpClient.Builder().addInterceptor { chain ->
            val req =
                chain.request().newBuilder().addHeader("Content-Type", "application/json").build()
            chain.proceed(req)
        }.build()

        return ApolloClient.Builder().serverUrl(url).okHttpClient(okHttpClient).build()
    }

    companion object {
        private const val BASE_URL = "http://localhost:3000/gql"
    }
}