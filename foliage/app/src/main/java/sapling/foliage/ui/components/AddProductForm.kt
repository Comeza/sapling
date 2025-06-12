package sapling.foliage.ui.components

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.material3.TextField
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.rememberCoroutineScope
import androidx.compose.runtime.setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import kotlinx.coroutines.launch
import me.zhanghai.compose.preference.defaultPreferenceFlow
import sapling.foliage.apolloClient
import sapling.foliage.gql.InsertProductMutation


@Composable
fun AddProductForm(
    ean: Long,
    onDismiss: () -> Unit = {}
) {
    var name by remember { mutableStateOf("") }
    val homeServer by defaultPreferenceFlow().collectAsState()
    val coroutineScope = rememberCoroutineScope()

    Column(modifier = Modifier.fillMaxWidth().padding(30.dp)) {
        Spacer(modifier = Modifier.height(120.dp))
        Text(
            "Add New Product Type",
            style = MaterialTheme.typography.headlineMedium,
            color = MaterialTheme.colorScheme.onBackground
        )
        Spacer(modifier = Modifier.height(16.dp))
        TextField(
            modifier = Modifier.fillMaxWidth(),
            value = name,
            onValueChange = { name = it },
            label = { Text("Name", color = MaterialTheme.colorScheme.onSurface) },
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
                coroutineScope.launch {
                    apolloClient(homeServer["home_server_url"] ?: "")
                        .mutation(
                            InsertProductMutation(
                                ean = ean,
                                name = name,
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