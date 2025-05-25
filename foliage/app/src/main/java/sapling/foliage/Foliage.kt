package sapling.foliage

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import me.zhanghai.compose.preference.ProvidePreferenceLocals
import sapling.foliage.ui.components.Inventory
import sapling.foliage.ui.components.MainNavbar
import sapling.foliage.ui.components.Settings
import sapling.foliage.ui.components.ShoppingTourList
import sapling.foliage.ui.screens.InventoryScreen
import sapling.foliage.ui.screens.SettingsScreen
import sapling.foliage.ui.screens.ShoppingScreen
import sapling.foliage.ui.theme.FoliageTheme

@Composable
fun FoliageApp() {
    FoliageTheme {
        ProvidePreferenceLocals {
            val navController = rememberNavController()
            Column(
                verticalArrangement = Arrangement.Bottom,
                modifier = Modifier.fillMaxSize()
            ) {
                NavHost(
                    navController = navController,
                    startDestination = ShoppingTourList,
                    modifier = Modifier.weight(1f)
                ) {
                    composable<ShoppingTourList> { ShoppingScreen() }
                    composable<Inventory> { InventoryScreen() }
                    composable<Settings> { SettingsScreen() }
                }
                MainNavbar(navController = navController)
            }
        }
    }
}