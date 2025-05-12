package sapling.foliage.ui.screens

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material3.FabPosition
import androidx.compose.material3.FloatingActionButton
import androidx.compose.material3.Icon
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import androidx.navigation.compose.rememberNavController
import sapling.foliage.ui.components.MainNavbar

@Composable
fun ShoppingScreen(modifier: Modifier = Modifier) {
    val navController = rememberNavController()
    Scaffold(
        modifier = Modifier.fillMaxSize(),
        floatingActionButton = {
            FloatingActionButton(onClick = {}) {
                Icon(Icons.Filled.Add, "Add new Shopping Tour")
            }
        },
    ) { _ ->
        Text("Shopping")
    }
//    Scaffold(
//        modifier= modifier.fillMaxSize(),
//        floatingActionButtonPosition = FabPosition.Start,
//        floatingActionButton = {
//            FloatingActionButton(onClick = {}) {
//                Icon(Icons.Filled.Add, "Add new Shopping Tour")
//            }
//        },
//    ) { _ -> Text("Shopping") }
}