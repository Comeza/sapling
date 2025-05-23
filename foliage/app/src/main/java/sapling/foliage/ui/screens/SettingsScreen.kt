package sapling.foliage.ui.screens

import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.outlined.Adb
import androidx.compose.material.icons.outlined.Info
import androidx.compose.material3.Icon
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.navigation.compose.rememberNavController
import me.zhanghai.compose.preference.ProvidePreferenceLocals
import me.zhanghai.compose.preference.switchPreference
import me.zhanghai.compose.preference.textFieldPreference

@Composable
fun SettingsScreen(modifier: Modifier = Modifier) {
    val navController = rememberNavController()
    Scaffold(
    ) { padding ->
        ProvidePreferenceLocals {
            LazyColumn(modifier = modifier, contentPadding = padding) {
                switchPreference(
                    key = "use_debug_server",
                    defaultValue = false,
                    title = { Text(text = "Use debug server") },
                    summary = { Text(text = if (it) "On" else "Off") })

                textFieldPreference(
                    key = "gql_endpoint",
                    title = { Text(text = "GQL Endpoint") },
                    defaultValue = "https://sapling.geigr.dev/gql",
                    summary = { Text(text = it) },
                    textToValue = { x -> x })


                textFieldPreference(
                    key = "debug_gql_endpoint",
                    title = { Text(text = "GQL Debug Endpoint") },
                    defaultValue = "http://sapling.local:3000/gql",
                    summary = { Text(text = it) },
                    textToValue = { x -> x })
            }
        }
    }
}
