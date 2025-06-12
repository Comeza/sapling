package sapling.foliage.ui.components

import android.widget.Toast
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.text.KeyboardOptions
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
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
import androidx.compose.ui.text.input.KeyboardType
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.launch
import me.zhanghai.compose.preference.defaultPreferenceFlow
import sapling.foliage.apolloClient
import sapling.foliage.gql.InsertItemMutation

@Composable
fun AddItemForm(
    ean: Long,
    onDismiss: () -> Unit = {}
) {
    var cost by remember { mutableStateOf("") }
    val context = LocalContext.current
    val homeServer by defaultPreferenceFlow().collectAsState()
    val coroutineScope = rememberCoroutineScope()
    val showProductForm = remember { mutableStateOf(false) }

    LaunchedEffect(ean) {
        try {
            val response = apolloClient(homeServer["home_server_url"] ?: "")
                .query(sapling.foliage.gql.ProductQuery(ean))
                .execute()
            // Add new ProductType
            if (response.data?.product == null) {
                android.util.Log.d("AddItemForm", "Product with EAN $ean does not exist, showing product form")
                showProductForm.value = true
                // quit function to avoid inserting item
            } else {
                android.util.Log.d("AddItemForm", "Product with EAN $ean exists")
            }

        } catch (e: Exception) {
            android.util.Log.e("AddItemForm", "Error checking product", e)
            Toast.makeText(
                context,
                "Error checking product: ${e.message}",
                Toast.LENGTH_SHORT
            )
                .show()
        }
    }

    if (showProductForm.value) {
        AddProductForm(
            ean = ean,
            onDismiss = { showProductForm.value = false }
        )
    } else {

        Column(
            modifier = Modifier.fillMaxWidth().padding(30.dp)) {
            Spacer(modifier = Modifier.height(120.dp))
            Text("Add Item", style = MaterialTheme.typography.headlineMedium)
            Spacer(modifier = Modifier.height(16.dp))
            TextField(
                modifier = Modifier.fillMaxWidth(),
                label = { Text("Cost (in Euro)") },
                value = cost,
                onValueChange = {cost = it},
                keyboardOptions = KeyboardOptions(keyboardType = KeyboardType.Number)
            )
            Spacer(modifier = Modifier.height(16.dp))
            Row (
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween
            ) {
                Button(onClick = { onDismiss() }) {
                    Text("Cancel")
                }
                Button(onClick = {

                    if (!cost.matches(Regex("^\\d{0,6}([.,]\\d{0,2})?$"))) {
                        Toast.makeText(context, "Please enter a valid cost", Toast.LENGTH_SHORT)
                            .show()
                        return@Button
                    }

                    val cent = ((cost.replace(',', '.').toDoubleOrNull() ?: 0.0) * 100).toInt()

                    coroutineScope.launch {
                        // check if Product with EAN exists
                        val response = apolloClient(homeServer["home_server_url"] ?: "")
                            .query(sapling.foliage.gql.ProductQuery(ean))
                            .execute()
                        if (response.data?.product == null) {
                            // If product does not exist prompt user to add product
                            showProductForm.value = true
                            // quit function to avoid inserting item
                            return@launch
                        }
                        // Insert Item
                        apolloClient(homeServer["home_server_url"] ?: "")
                            .mutation(
                                InsertItemMutation(
                                    ean = ean,
                                    cost = cent,
                                )
                            )
                            .execute()
                        onDismiss()
                    }
                }) {
                    Text("Add")
                }
            }
        }
    }
}