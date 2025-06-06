package sapling.foliage

import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import com.apollographql.apollo.ApolloClient
import me.zhanghai.compose.preference.defaultPreferenceFlow

fun apolloClient(serverUrl: String?): ApolloClient =
    ApolloClient.Builder().serverUrl(serverUrl ?: "https://sapling.geigr.dev/gql")
        .build()
