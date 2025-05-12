package sapling.foliage

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material3.Icon
import androidx.compose.material3.NavigationBar
import androidx.compose.material3.NavigationBarItem
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.navigation.NavDestination.Companion.hasRoute
import androidx.navigation.NavDestination.Companion.hierarchy
import androidx.navigation.NavGraph.Companion.findStartDestination
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.currentBackStackEntryAsState
import androidx.navigation.compose.rememberNavController
import kotlinx.serialization.Serializable
import sapling.foliage.ui.components.Inventory
import sapling.foliage.ui.components.MainNavbar
import sapling.foliage.ui.components.Settings
import sapling.foliage.ui.components.ShoppingTourList
import sapling.foliage.ui.screens.InventoryScreen
import sapling.foliage.ui.screens.SettingsScreen
import sapling.foliage.ui.screens.ShoppingScreen
import sapling.foliage.ui.theme.FoliageTheme


class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            FoliageTheme {
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
}