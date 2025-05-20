package sapling.foliage.ui.components

import android.widget.Toast
import androidx.compose.material3.Button
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.platform.LocalContext
import com.google.mlkit.vision.barcode.common.Barcode
import com.google.mlkit.vision.codescanner.GmsBarcodeScannerOptions
import com.google.mlkit.vision.codescanner.GmsBarcodeScanning

@Composable
fun BarcodeScanner() {
    val context = LocalContext.current
    val scannerOptions = remember {
        GmsBarcodeScannerOptions.Builder()
            .setBarcodeFormats(
                Barcode.FORMAT_EAN_13,
            )
            .enableAutoZoom()
            .build()
    }
    val scanner = remember { GmsBarcodeScanning.getClient(context, scannerOptions) }
    var scannedValue by remember { mutableStateOf<String?>(null) }

    Button(onClick = {
        scanner.startScan()
            .addOnSuccessListener { barcode ->
                scannedValue = barcode.rawValue
            }
            .addOnCanceledListener {
                Toast.makeText(context, "Scan canceled", Toast.LENGTH_SHORT).show()
            }
            .addOnFailureListener {
                Toast.makeText(context, "Scan failed: ${it.message}", Toast.LENGTH_SHORT)
                    .show()
            }
    }) {
        Text("Scan Barcode")
    }

    scannedValue?.let {
        Text("Scanned value: $it")
    }
}