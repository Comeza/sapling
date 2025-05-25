package sapling.foliage.ui.components

import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Kitchen
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material.icons.filled.ShoppingCart
import androidx.compose.material.icons.outlined.Kitchen
import androidx.compose.material.icons.outlined.Settings
import androidx.compose.material.icons.outlined.ShoppingCart
import androidx.compose.material3.Icon
import androidx.compose.material3.NavigationBar
import androidx.compose.material3.NavigationBarItem
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.vector.ImageVector
import androidx.navigation.NavController
import androidx.navigation.NavDestination.Companion.hasRoute
import androidx.navigation.NavDestination.Companion.hierarchy
import androidx.navigation.NavGraph.Companion.findStartDestination
import androidx.navigation.compose.currentBackStackEntryAsState
import kotlinx.serialization.Serializable

data class NavItem<T : Any>(
    val label: String,
    val selectedIcon: ImageVector,
    val defaultIcon: ImageVector,
    val route: T
)

val navItemList = listOf(
    NavItem(
        label = "Einkäufe",
        selectedIcon = Icons.Filled.ShoppingCart,
        defaultIcon = Icons.Outlined.ShoppingCart,
        route = ShoppingTourList
    ),
    NavItem(
        label = "Vorat",
        selectedIcon = Icons.Filled.Kitchen,
        defaultIcon = Icons.Outlined.Kitchen,
        route = Inventory
    ),
    NavItem(
        label = "Settings",
        selectedIcon = Icons.Filled.Settings,
        defaultIcon = Icons.Outlined.Settings,
        route = Settings
    ),
)

@Serializable object ShoppingTourList
@Serializable object Inventory
@Serializable object Settings


@Composable
fun MainNavbar(modifier: Modifier = Modifier, navController: NavController) {

    NavigationBar {
        val navBackStackEntry by navController.currentBackStackEntryAsState()
        val currentDestination = navBackStackEntry?.destination
        navItemList.forEach { topLevelRoute ->
            val isSelected = currentDestination?.hierarchy?.any { it.hasRoute(topLevelRoute.route::class) } == true
            NavigationBarItem(
                icon = {
                    Icon(
                        imageVector = if (isSelected) topLevelRoute.selectedIcon else topLevelRoute.defaultIcon,
                        contentDescription = topLevelRoute.label
                    )
                },
                label = { Text(topLevelRoute.label) },
                selected = isSelected,
                onClick = {
                    navController.navigate(topLevelRoute.route) {
                        // Pop up to the start destination of the graph to
                        // avoid building up a large stack of destinations
                        // on the back stack as users select items
                        popUpTo(navController.graph.findStartDestination().id) {
                            saveState = true
                        }
                        // Avoid multiple copies of the same destination when
                        // reselecting the same item
                        launchSingleTop = true
                        // Restore state when reselecting a previously selected item
                        restoreState = true
                    }
                }
            )
        }
    }
}