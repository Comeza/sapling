package sapling.foliage.ui.screens

import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Home
import androidx.compose.material3.Icon
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.preference.PreferenceScreen
import me.zhanghai.compose.preference.textFieldPreference

@Composable
fun SettingsScreen(modifier: Modifier = Modifier) {
    Scaffold(modifier) { contentPadding ->
        LazyColumn(modifier = Modifier.fillMaxSize(), contentPadding = contentPadding) {
            textFieldPreference(
                key = "home_server_url",
                defaultValue = "https://sapling.geigr.dev/gql",
                icon = {
                    Icon(
                        imageVector = Icons.Filled.Home,
                        contentDescription = null
                    )
                },
                title = { Text(text = "Home Server URL") },
                summary = { Text(it) },
                textToValue = { it }
            )
        }
    }
}
