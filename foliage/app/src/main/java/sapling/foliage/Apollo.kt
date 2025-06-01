package sapling.foliage

import com.apollographql.apollo.ApolloClient

val apolloClient = ApolloClient.Builder().serverUrl("https://sapling.geigr.dev/gql")
    .build()
