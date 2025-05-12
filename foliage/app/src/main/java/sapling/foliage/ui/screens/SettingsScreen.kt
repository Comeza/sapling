package sapling.foliage.ui.screens

import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.navigation.compose.rememberNavController
import sapling.foliage.ui.components.MainNavbar

@Composable
fun SettingsScreen(modifier: Modifier = Modifier) {
    val navController = rememberNavController()
    Scaffold(
    ){ _ ->
        Text("Settings")
    }
}