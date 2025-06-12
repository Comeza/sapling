package sapling.foliage.ui.screens

import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Add
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.FloatingActionButton
import androidx.compose.material3.Icon
import androidx.compose.material3.ListItem
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.material3.pulltorefresh.PullToRefreshBox
import androidx.compose.material3.pulltorefresh.pullToRefresh
import androidx.compose.material3.pulltorefresh.rememberPullToRefreshState
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.material3.Button
import androidx.navigation.compose.rememberNavController
import kotlinx.coroutines.launch
import me.zhanghai.compose.preference.defaultPreferenceFlow
import sapling.foliage.apolloClient
import sapling.foliage.gql.InventoryQuery
import sapling.foliage.gql.DeleteItemMutation
import sapling.foliage.ui.components.AddItemForm
import sapling.foliage.ui.components.BarcodeScanner

@OptIn(ExperimentalMaterial3Api::class)
@Composable
@Preview
fun InventoryScreen(modifier: Modifier = Modifier) {
    val navController = rememberNavController()

    var itemList by remember { mutableStateOf(emptyList<InventoryQuery.Item>()) }
    var isRefreshing by remember { mutableStateOf(false) }
    val state = rememberPullToRefreshState()
    val coroutineScope = rememberCoroutineScope()
    val homeServer by defaultPreferenceFlow().collectAsState()

    fun fetchData() {
        isRefreshing = true
        coroutineScope.launch {
            val response =
                apolloClient(homeServer["home_server_url"]).query(InventoryQuery()).execute()
            itemList = response.data?.items ?: emptyList()
            isRefreshing = false
        }
    }

    var showAddItemForm by remember { mutableStateOf(false) }
    var scannedEan by remember { mutableStateOf("") }
    val context = LocalContext.current

    LaunchedEffect(homeServer) {
        fetchData()
    }

    Scaffold(
        modifier = Modifier.pullToRefresh(
            state = state,
            isRefreshing = isRefreshing,
            onRefresh = { fetchData() }
        ),
        floatingActionButton = {
            if (!showAddItemForm) {
                FloatingActionButton(onClick = { BarcodeScanner.scan(context, { ean ->
                    scannedEan = ean
                    showAddItemForm = true
                }, {})}) {
                    Icon(Icons.Filled.Add, "Insert Items")
                }
            }
        },

        topBar = { TopAppBar(title = { Text("Inventory") }) }
    ) {
        if (showAddItemForm) {
            AddItemForm(
                ean = scannedEan.toLongOrNull() ?: 0L,
                onDismiss = { showAddItemForm = false }
            )
        } else {
            ItemList(
                modifier = Modifier
                    .fillMaxSize()
                    .padding(it),
                items = itemList,
            )
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ItemList(
    items: List<InventoryQuery.Item>,
    modifier: Modifier = Modifier,
    isRefreshing: Boolean = false,
    onRefresh: () -> Unit = {},
) {
    PullToRefreshBox(
        modifier = modifier,
        onRefresh = onRefresh,
        isRefreshing = isRefreshing,
    ) {
        LazyColumn(Modifier.fillMaxSize()) {
            items(items) {
                InventoryItem(it)
            }
        }
    }
}

@Composable
fun InventoryItem(
    item: InventoryQuery.Item,
    modifier: Modifier = Modifier
) {
    val homeServer by defaultPreferenceFlow().collectAsState()
    val coroutineScope = rememberCoroutineScope()

    ListItem(
        modifier = modifier,
        headlineContent = { Text(item.product.name) },
        trailingContent = {
            Button(onClick = {
                coroutineScope.launch {
                    apolloClient(homeServer["home_server_url"] ?: "")
                            .mutation(DeleteItemMutation(itemId = item.itemId))
                            .execute()
                }
            }) {
                Text("Delete")
            }
        }
    )
}