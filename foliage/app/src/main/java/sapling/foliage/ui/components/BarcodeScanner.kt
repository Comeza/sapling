package sapling.foliage.ui.components

import android.widget.Toast
import com.google.mlkit.vision.barcode.common.Barcode
import com.google.mlkit.vision.codescanner.GmsBarcodeScannerOptions
import com.google.mlkit.vision.codescanner.GmsBarcodeScanning

class BarcodeScanner {

    companion object {
        fun scan(context: android.content.Context, onSuccess: (String) -> Unit, onError: (String) -> Unit) {
            val scannerOptions =
                GmsBarcodeScannerOptions.Builder()
                    .setBarcodeFormats(
                        Barcode.FORMAT_EAN_13,
                    )
                    .enableAutoZoom()
                    .build()

            GmsBarcodeScanning.getClient(context, scannerOptions).startScan()
                .addOnSuccessListener { barcode ->
                    onSuccess(barcode.rawValue ?: "")
                }
                .addOnCanceledListener {
                    Toast.makeText(context, "Scan canceled", Toast.LENGTH_SHORT).show()
                    onError("")
                }
                .addOnFailureListener {
                    Toast.makeText(context, "Scan failed: ${it.message}", Toast.LENGTH_SHORT)
                        .show()
                    onError("")
                }
        }
    }
}